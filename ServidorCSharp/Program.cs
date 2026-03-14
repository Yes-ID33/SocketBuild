using System;
using System.Net;
using System.Net.Sockets;
using System.IO;

class Program {
    static void Main() {
        int puerto = 7084;
        // IPAddress.Any permite escuchar en todas las interfaces de red
        TcpListener server = new TcpListener(IPAddress.Parse("127.0.0.1"), puerto);

        try {
            server.Start();
            Console.WriteLine($"Servidor C# escuchando en el puerto {puerto}...");

            // Aceptamos la conexión
            using (TcpClient client = server.AcceptTcpClient())
            using (NetworkStream stream = client.GetStream())
            using (StreamReader reader = new StreamReader(stream))
            using (StreamWriter writer = new StreamWriter(stream) { AutoFlush = true }) {

                Console.WriteLine($"¡Conectado con el cliente: {client.Client.RemoteEndPoint}!");

                string? mensaje;
                while ((mensaje = reader.ReadLine()) != null) {
                    Console.WriteLine($"Cliente dice: {mensaje}");

                    int longitud = mensaje.Length;
                    string respuesta;

                    if (longitud > 15)
                        respuesta = $"Que mensaje tan largo: '{mensaje}'";
                    else if (longitud > 8 && longitud < 16)
                        respuesta = $"Mensaje de tamaño promedio: '{mensaje}'";
                    else
                        respuesta = $"Que mensaje tan pequeño: '{mensaje}'";

                    writer.WriteLine(respuesta);
                }
            }
        } catch (Exception e) {
            Console.WriteLine($"Error: {e.Message}");
        } finally {
            server.Stop();
            Console.WriteLine("Servidor apagado.");
        }
    }
}