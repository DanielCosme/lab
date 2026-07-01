#include <cstdint>
#include <cstring>
#include <stdio.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <unistd.h>

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
one_request(int conn_socket_handle)
{
	do_something(conn_socket_handle);
	return 0;
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
							printf("message received from: %d:%d\n", ntohl(addr.sin_addr.s_addr), ntohs(addr.sin_port));
							while(true)
							{
								int32_t err = one_request(conn_socket_handle);
								if (err)
								{
									printf("error\n");
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
