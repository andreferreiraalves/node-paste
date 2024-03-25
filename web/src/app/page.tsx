import MainPage from "./main_page";

export default function Home() {
  return (
    <>
      <nav className="navbar navbar-expand-lg">
        <div className="container-xl">
          <a className="navbar-brand" href="#">
            Navbar
          </a>

          <div className="collapse navbar-collapse" id="navbarNav"></div>
        </div>
      </nav>

      <MainPage />
    </>
  );
}
