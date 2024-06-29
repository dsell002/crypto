import hashlib

def sha256(string):
    sha_signature = hashlib.sha256(string.encode()).hexdigest()
    return sha_signature

if __name__ == "__main__":
    string = "Hello, world!"
    print(f"SHA-256 of '{string}': {sha256(string)}")
