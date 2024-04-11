import MainPage from "../main_page";

export default function Page({ params }: { params: { id?: string } }) {
  return (
    <>
      <MainPage id={params.id} />
    </>
  );
}
