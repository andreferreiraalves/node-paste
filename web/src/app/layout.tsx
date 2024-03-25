import "bootstrap/dist/css/bootstrap.min.css";
import BootstrapClient from "./BootstrapClient";
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
      {/* <script src="bootstrap/dist/js/bootstrap.bundle.js"></script> */}
      <body>
        {children}
        <BootstrapClient />
      </body>
    </html>
  );
}
