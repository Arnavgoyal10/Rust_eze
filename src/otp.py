import sys
import pyotp
import psycopg2
from psycopg2.extras import RealDictCursor


# Function to generate a unique TOTP secret for an account
def generate():
    secret = pyotp.random_base32()
    return secret


# Function to verify the TOTP during login
def verify(secret, totp_code):
    totp = pyotp.TOTP(secret)
    return totp.verify(totp_code)


# Add command-line argument handling
if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Error: No command specified", file=sys.stderr)
        sys.exit(1)

    command = sys.argv[1]
    
    if command == "generate":
        print(generate())
    elif command == "verify":
        if len(sys.argv) != 4:
            print("Error: verify requires secret and TOTP code", file=sys.stderr)
            sys.exit(1)
        secret = sys.argv[2]
        totp_code = sys.argv[3]
        print(str(verify(secret, totp_code)).lower())  # Python's True/False to "true"/"false"
    else:
        print(f"Error: Unknown command {command}", file=sys.stderr)
        sys.exit(1)

