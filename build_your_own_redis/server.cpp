#include <cassert>
#include <cerrno>
#include <cstdint>
#include <cstring>
#include <stdio.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <unistd.h>

const size_t k_max_msg = 4096;

static void
msg(const char *msg)
{
    fprintf(stderr, "%s\n", msg);
}

static void
do_something(int conn_socket_handle)
{
	char rbuf[64] = {};
	ssize_t n = read(conn_socket_handle, rbuf, sizeof(rbuf) - 1);
	if (n)
	{
		printf("client says: %s\n", rbuf);
		char wbuf[] = "world";
		write(conn_socket_handle, wbuf, strlen(wbuf));
	}
}

static int32_t
read_full(int socket_handle, char *buf, size_t buf_size)
{
	while (buf_size > 0)
	{
		ssize_t bytes_read = read(socket_handle, buf, buf_size);
		if (bytes_read > 0)
		{
			assert((size_t)bytes_read <= buf_size);
			buf_size -= (size_t)bytes_read;
			buf += bytes_read;
		}
		else
		{
			return -1; // error
		}
	}
	return 0;
}

static int32_t
write_all(int socket_handle, const char *buf, size_t buf_size)
{
	while (buf_size > 0)
	{
		ssize_t bytes_read = write(socket_handle, buf, buf_size);
		if (bytes_read > 0)
		{
			assert((size_t)bytes_read <= buf_size);
			buf_size -= (size_t)bytes_read;
			buf += bytes_read;
		}
		else
		{
			return -1; // error
		}
	}
	return 0;
}

static int32_t
one_request(int conn_socket_handle)
{
	// bytes header
	char rbuf[4 + k_max_msg];
	errno = 0;
	int32_t err = read_full(conn_socket_handle, rbuf, 4);
	if (err == 0)
	{
		uint32_t len = 0;
		memcpy(&len, rbuf, 4); // Assume little endian
		if (len < k_max_msg)
		{
			// Request body
			err = read_full(conn_socket_handle, &rbuf[4], len);
			if (err == 0)
			{
				// Do something
				printf("client says: %.*s\n", len, &rbuf[4]);
				// Reply using the same protocol
				const char reply[] = "world";
				char wbuf[4 + sizeof(reply)];
				len = (uint32_t)strlen(reply);
				memcpy(wbuf, &len, 4);
				memcpy(&wbuf[4], reply, len);
				return write_all(conn_socket_handle, wbuf, 4 + len);
			}
			else
			{
				msg("read() error");
			}
		}
		else
		{
			msg("too long");
		}
	}
	else
	{
		msg(errno == 0 ? "EOF" : "read() error");
		return err;
	}
	return -1;
}

int
main()
{
	int socket_handle = socket(AF_INET, SOCK_STREAM, 0);
	if (socket_handle)
	{
		printf("socket handle created: %d\n", socket_handle);
		int val = 1;
		if (setsockopt(socket_handle, SOL_SOCKET, SO_REUSEADDR, &val, sizeof(val)) == 0)
		{
			printf("socket options set\n");
			int port = 1234;
			int address = 0;
			struct sockaddr_in addr = {};
			addr.sin_family = AF_INET;
			addr.sin_port = htons(port);
			addr.sin_addr.s_addr = htonl(address); // From LE (host) to BE (network) (Host To Network Long)
			if (bind(socket_handle, (const struct sockaddr *)&addr, sizeof(addr)) == 0)
			{
				printf("socket bound to: %d:%d\n", ntohl(addr.sin_addr.s_addr), ntohs(addr.sin_port));
				if (listen(socket_handle, SOMAXCONN) == 0)
				{
					printf("socket listening\n");
					while (true)
					{
						struct sockaddr_in  client_addr = {};
						socklen_t addrlen = sizeof(client_addr);
						int conn_socket_handle = accept(socket_handle, (struct sockaddr *)&client_addr, &addrlen);
						if (conn_socket_handle)
						{
							// printf("message received from: %d:%d\n", ntohl(addr.sin_addr.s_addr), ntohs(addr.sin_port));
							while(true)
							{
								int32_t err = one_request(conn_socket_handle);
								if (err)
								{
									break;
								}
							}
						}
						close(conn_socket_handle);
					}
				}
			}
		}
	}
}
