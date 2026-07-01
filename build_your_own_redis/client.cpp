#include <cstring>
#include <stdio.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <unistd.h>

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
			char msg[] = "hello";
			write(socket_handle, msg, strlen(msg));
			char rbuf[64] = {};
			ssize_t n = read(socket_handle, rbuf, sizeof(rbuf) - 1);
			if (n > 0)
			{
				printf("server says: %s\n", rbuf);
				close(socket_handle);
			}
		}
	}
}
