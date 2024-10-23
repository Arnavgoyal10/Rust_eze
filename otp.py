import sys
import pyotp
import psycopg2
from psycopg2.extras import RealDictCursor


# Function to generate a unique TOTP secret for an account
def generate_totp_secret(account_id):
    secret = pyotp.random_base32()
    # Store the secret in the database
    conn = psycopg2.connect("dbname=your_db user=your_user password=your_password")
    cursor = conn.cursor()
    cursor.execute(
        "UPDATE accounts SET totp_secret = %s WHERE id = %s", (secret, account_id)
    )
    conn.commit()
    cursor.close()
    conn.close()
    return secret


# Function to verify the TOTP during login
def verify_totp(username, totp_code):
    conn = psycopg2.connect("dbname=your_db user=your_user password=your_password")
    cursor = conn.cursor(cursor_factory=RealDictCursor)
    cursor.execute(
        "SELECT totp_secret FROM accounts WHERE account_holder_name = %s", (username,)
    )
    result = cursor.fetchone()
    cursor.close()
    conn.close()

    if result:
        secret = result["totp_secret"]
        totp = pyotp.TOTP(secret)
        return totp.verify(totp_code)
    return False


def main():
    if len(sys.argv) < 3:
        print("Usage: python otp.py <command> <args>")
        return

    command = sys.argv[1]

    if command == "generate":
        account_id = sys.argv[2]
        secret = generate_totp_secret(account_id)
        print(f"TOTP secret for account {account_id}: {secret}")

    elif command == "verify":
        username = sys.argv[2]
        totp_code = sys.argv[3]
        if verify_totp(username, totp_code):
            print("TOTP verification successful.")
            sys.exit(0)
        else:
            print("TOTP verification failed.")
            sys.exit(1)


if __name__ == "__main__":
    main()
