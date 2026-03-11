import socket

HOST = '127.0.0.1'
PORT = 7085

with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as server_socket:
    server_socket.bind((HOST, PORT))
    server_socket.listen()
    print(f"Servidor Python escuchando en {HOST}:{PORT}...")

    conn, addr = server_socket.accept()

    with conn:
        print(f"¡Conectado exitosamente con el cliente: {addr}!")

        # Iniciamos el ciclo de escucha continua
        while True:
            data = conn.recv(1024)

            # Si data está vacío, significa que el cliente cerró el socket
            if not data:
                print("El cliente ha cerrado la conexión.")
                break # Rompemos el ciclo while

            mensaje_recibido = data.decode('utf-8')
            longitud = len(mensaje_recibido)
            print(f"Cliente dice: {mensaje_recibido}")

            # Enviamos una respuesta confirmando lo que recibimos
            if longitud>15:
                respuesta = f"Que mensaje tan largo: '{mensaje_recibido}'"
                conn.sendall(respuesta.encode('utf-8'))
            elif 8 < longitud < 16:
                respuesta = f"Mensaje de tamaño promedio: '{mensaje_recibido}'"
                conn.sendall(respuesta.encode('utf-8'))
            else:
                respuesta = (f"Que mensaje tan pequeño: '{mensaje_recibido}'")
                conn.sendall(respuesta.encode('utf-8'))

print("Servidor apagado.")