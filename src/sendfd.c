#include <string.h>
#include <sys/socket.h>

int sendfd(int socket, int fd) {
    struct msghdr msg;
    memset(&msg, 0, sizeof(msg));
    char buf[CMSG_SPACE(sizeof(fd))];
    memset(buf, '\0', sizeof(buf));

    /* On Mac OS X, the struct iovec is needed, even if it points to minimal data */
    struct iovec io = { .iov_base = "", .iov_len = 1 };

    msg.msg_iov = &io;
    msg.msg_iovlen = 1;
    msg.msg_control = buf;
    msg.msg_controllen = sizeof(buf);

    struct cmsghdr * cmsg = CMSG_FIRSTHDR(&msg);
    cmsg->cmsg_level = SOL_SOCKET;
    cmsg->cmsg_type = SCM_RIGHTS;
    cmsg->cmsg_len = CMSG_LEN(sizeof(fd));

    memmove(CMSG_DATA(cmsg), &fd, sizeof(fd));

    msg.msg_controllen = cmsg->cmsg_len;

    return sendmsg(socket, &msg, 0);
}

int recvfd(int socket) {
    int err;
    struct msghdr msg;
    memset(&msg, 0, sizeof(msg));

    /* On Mac OS X, the struct iovec is needed, even if it points to minimal data */
    char m_buffer[1];
    struct iovec io = { .iov_base = m_buffer, .iov_len = sizeof(m_buffer) };
    msg.msg_iov = &io;
    msg.msg_iovlen = 1;

    char c_buffer[256];
    msg.msg_control = c_buffer;
    msg.msg_controllen = sizeof(c_buffer);

    if ((err = recvmsg(socket, &msg, 0)) < 0)
        return err;

    struct cmsghdr *cmsg = CMSG_FIRSTHDR(&msg);

    int fd;
    memmove(&fd, CMSG_DATA(cmsg), sizeof(fd));
    return fd;
}
