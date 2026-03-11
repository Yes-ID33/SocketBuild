using System;
using System.Net.Sockets;
using System.IO;

class Program {
    static void Main() {
        string host = "127.0.0.1";
        int puerto = 7084;

        try {
            using (TcpClient client = new TcpClient(host, puerto))
            using (NetworkStream stream = client.GetStream())
            using (StreamReader reader = new StreamReader(stream))
            using (StreamWriter writer = new StreamWriter(stream) { AutoFlush = true }) {
                
                Console.WriteLine("¡Conectado al servidor C#!");

                while (true) {
                    Console.Write("Escribe tu mensaje: ");
                    string mensaje = Console.ReadLine();
                    
                    writer.WriteLine(mensaje);

                    string respuesta = reader.ReadLine();
                    if (respuesta == null) break;
                    
                    Console.WriteLine($"Servidor responde: {respuesta}");

                    Console.Write("¿Cerrar canal? (s/n): ");
                    if (Console.ReadLine().Trim().ToLower() == "s") break;
                    Console.WriteLine("------------------------------");
                }
            }
        } catch (Exception e) {
            Console.WriteLine($"Error: {e.Message}");
        }
        Console.WriteLine("Cliente finalizado.");
    }
}