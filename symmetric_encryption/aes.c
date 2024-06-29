#include <stdio.h>
#include <string.h>
#include <openssl/aes.h>

void aes_encrypt(const unsigned char *key, const unsigned char *input, unsigned char *output) {
    AES_KEY encryptKey;
    AES_set_encrypt_key(key, 128, &encryptKey);
    AES_encrypt(input, output, &encryptKey);
}

void aes_decrypt(const unsigned char *key, const unsigned char *input, unsigned char *output) {
    AES_KEY decryptKey;
    AES_set_decrypt_key(key, 128, &decryptKey);
    AES_decrypt(input, output, &decryptKey);
}

int main() {
    unsigned char key[16] = "mysecretkey12345"; // 128-bit key
    unsigned char input[16] = "plaintextinput!"; // 16 bytes block
    unsigned char encrypted[16];
    unsigned char decrypted[16];

    aes_encrypt(key, input, encrypted);
    aes_decrypt(key, encrypted, decrypted);

    printf("Original:   %s\n", input);
    printf("Encrypted:  ");
    for (int i = 0; i < 16; i++) {
        printf("%02x", encrypted[i]);
    }
    printf("\nDecrypted:  %s\n", decrypted);

    return 0;
}
