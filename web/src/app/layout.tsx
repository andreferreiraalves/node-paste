// import "bootstrap/dist/css/bootstrap.min.css";
// import BootstrapClient from "./BootstrapClient";
import TailwindClient from "./TailwindClient";
import "./global.css";

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="pt-br">
      <head>
        <meta name="viewport" content="width=device-width, initial-scale=1" />
      </head>
      <body>
        {children}
        <TailwindClient />
      </body>
    </html>
  );
}
