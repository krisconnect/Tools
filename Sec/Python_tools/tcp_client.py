import socket

target_host="127.0.0.1"
target_port=9000

#Create socket object
client = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

#Connect the client
client.connect((target_host, target_port))

#Send data
client.send(b"GET / HTTP/1.1\r\nHOST:google.com\r\n\r\n")

#Recieve data
response = client.recv(4096)

print(response)
