import socket

# 1. Configuración de red
# Debe coincidir exactamente con la IP y el puerto donde el servidor está escuchando
HOST = '127.0.0.1'  # La IP del servidor (localhost)
PORT = 7084        # El puerto del servidor

# 2. Creación del socket del cliente
# Usamos la misma configuración: IPv4 (AF_INET) y TCP (SOCK_STREAM)
with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as client_socket:
    print(f"Intentando conectar al servidor en {HOST}:{PORT}...")

    client_socket.connect((HOST, PORT))
    print("¡Conexión establecida!\n")

    while True:
        mensaje = input("Escribe tu mensaje: ")
        client_socket.sendall(mensaje.encode('utf-8'))
        # 5. Recibir respuesta (Recv)
        # Esperamos la respuesta del servidor (hasta 1024 bytes)
        data = client_socket.recv(1024)
        # Decodificamos los bytes de vuelta a texto para leerlos
        print(f"Respuesta del servidor: '{data.decode('utf-8')}'")

        salir = input("Deseas cerrar el chat? (s/n)")
        if salir == 's':
            print("Cerrando la conexión...")
            break
        print("--|--"*20)

print("Canal cerrado.")