import socket
import json

# Define the path to the Unix Domain Socket
SOCKET_PATH = "/tmp/merrimack.sock"

# Construct the message
message = {
    "interval_minutes": 1,
    "duration_seconds": 10
}

# Serialize to JSON
serialized_message = json.dumps(message).encode('utf-8')

# Create a Unix Domain Socket
client_socket = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)

try:
    # Connect to the socket
    client_socket.connect(SOCKET_PATH)

    # Send the serialized JSON
    client_socket.sendall(serialized_message)

    print("Message sent successfully.")

except Exception as e:
    print(f"Failed to send message: {e}")

finally:
    # Close the socket
    client_socket.close()

