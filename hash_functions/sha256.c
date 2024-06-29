#include <stdio.h>
#include <string.h>
#include <openssl/sha.h>

void sha256(const char *string, char outputBuffer[65]) {
    unsigned char hash[SHA256_DIGEST_LENGTH];
    SHA256_CTX sha256;
    SHA256_Init(&sha256);
    SHA256_Update(&sha256, string, strlen(string));
    SHA256_Final(hash, &sha256);
    
    for(int i = 0; i < SHA256_DIGEST_LENGTH; i++) {
        sprintf(outputBuffer + (i * 2), "%02x", hash[i]);
    }
    outputBuffer[64] = 0;
}

int main() {
    char *string = "Hello, world!";
    char outputBuffer[65];
    sha256(string, outputBuffer);
    printf("SHA-256 of \"%s\": %s\n", string, outputBuffer);
    return 0;
}
