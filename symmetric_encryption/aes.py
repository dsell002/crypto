from Crypto.Cipher import AES
from Crypto.Util.Padding import pad, unpad

def aes_encrypt(key, data):
    cipher = AES.new(key, AES.MODE_ECB)
    encrypted = cipher.encrypt(pad(data, AES.block_size))
    return encrypted

def aes_decrypt(key, data):
    cipher = AES.new(key, AES.MODE_ECB)
    decrypted = unpad(cipher.decrypt(data), AES.block_size)
    return decrypted

if __name__ == "__main__":
    key = b'mysecretkey12345'  # 128-bit key
    data = b'plaintextinput!'  # 16 bytes block

    encrypted = aes_encrypt(key, data)
    decrypted = aes_decrypt(key, encrypted)

    print(f"Original:   {data}")
    print(f"Encrypted:  {encrypted.hex()}")
    print(f"Decrypted:  {decrypted}")
