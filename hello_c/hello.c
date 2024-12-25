#include <stdio.h>
#include <time.h>
//#include <arpa/inet.h>
//#include <netinet/in.h>
//#include <sys/socket.h>

//const char header[] = "\
//HTTP/1.1 200 OK\r\n\
//Content-Type: text/html\r\n\
//Content-Length: %u\r\n\
//Connection: close\r\n\
//\r\n\
//";
//
//const char content[] = "<html>\n\
//<head>\n\
//  <title>Hello, ArceOS</title>\n\
//</head>\n\
//<body>\n\
//  <center>\n\
//    <h1>Hello, <a href=\"https://github.com/arceos-org/arceos\">ArceOS</a></h1>\n\
//  </center>\n\
//  <hr>\n\
//  <center>\n\
//    <i>Powered by <a href=\"https://github.com/arceos-org/arceos/tree/main/examples/httpserver-c\">ArceOS example HTTP server</a> v0.1.0</i>\n\
//  </center>\n\
//</body>\n\
//</html>\n\
//";

int main(){
//    puts("Hello, ArceOS C HTTP server!");
//    struct sockaddr_in local, remote;
//    int addr_len = sizeof(remote);
//    local.sin_family = AF_INET;
//    if (inet_pton(AF_INET, "0.0.0.0", &(local.sin_addr)) != 1) {
//        perror("inet_pton() error");
//        return -1;
//    }

    int start,end;
    start=clock();
    puts("hello");
    end=clock();
    printf("start:%d,end:%d,time:%dms",start,end,end-start);
    return 0;
}

