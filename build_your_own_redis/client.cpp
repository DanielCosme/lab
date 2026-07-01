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
query(int socket_handle, const char *text)
{
	int32_t err = -1;
	uint32_t len = (uint32_t)strlen(text);
	if (len < k_max_msg)
	{
		// Send request
		char wbuf[4 + k_max_msg];
		memcpy(wbuf, &len, 4); // assume little endian
		memcpy(&wbuf[4], text, len);
		err = write_all(socket_handle, wbuf, 4 + len);
		if (!err)
		{
			// 4 bytes header
			char rbuf[4 + k_max_msg];
			errno = 0;
			err = read_full(socket_handle, rbuf, 4);
			if (!err)
			{
				memcpy(&len, rbuf, 4); // assume little endian
				if (len < k_max_msg)
				{
					err = read_full(socket_handle, &rbuf[4], len);
					if (!err)
					{
						// So something
						printf("server says: %.*s\n", len, &rbuf[4]);
						return 0;
					}
					msg("read() error");
				}
				else
				{
					msg("too long");
					return -1;
				}
			}
			else
			{
				// TODO: handle:
				// read can also be interrupted by a signal because it must wait if the buffer is empty.
				// In this case, 0 bytes are read, but the return value is -1 and errno is EINTR.
				msg(errno == 0 ? "EOF" : "read() error");
				return err;
			}
		}
	}
	return err;
}

int
main()
{
	int socket_handle = socket(AF_INET, SOCK_STREAM, 0);
	if (socket_handle)
	{
		printf("client: socket handle created: %d\n", socket_handle);
		int port = 1234;
		struct sockaddr_in addr = {};
		addr.sin_family = AF_INET;
		addr.sin_port = ntohs(port);
		addr.sin_addr.s_addr = ntohl(INADDR_LOOPBACK);
		int rv = connect(socket_handle, (const struct sockaddr *)&addr, sizeof(addr));
		if (rv == 0)
		{
			printf("client connected to server\n");
			int32_t err = query(socket_handle, "hello1");
			if (!err)
			{
				err = query(socket_handle, "hello2");
			}
			close(socket_handle);
		}
	}
}
