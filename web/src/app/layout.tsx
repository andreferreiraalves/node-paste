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
        <nav className="bg-white border-gray-200 dark:bg-gray-900">
          <div className="flex flex-wrap items-center justify-between mx-auto p-4">
            <a
              href="https://flowbite.com/"
              className="flex items-center space-x-3 rtl:space-x-reverse"
            >
              <img
                src="https://flowbite.com/docs/images/logo.svg"
                className="h-8"
                alt="Flowbite Logo"
              />
              <span className="self-center text-2xl font-semibold whitespace-nowrap dark:text-white">
                Paste
              </span>
            </a>
          </div>
        </nav>
        {children}
        <TailwindClient />
      </body>
    </html>
  );
}
