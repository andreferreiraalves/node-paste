"use client";

import getApiUrl from "@/helpers/api";
import { ChangeEvent, useState } from "react";

export default function MainPage() {
  const [message, setMessage] = useState("");

  const onChange = (e: ChangeEvent<HTMLTextAreaElement>) => {
    setMessage(e.target.value);
  };

  const send = async () => {
    const result = await fetch(getApiUrl(), {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify( {
        message
      })

    });
    const response = await result.json()

    navigator.clipboard.writeText("ronaldo");
    alert("Link copiado para clipbloard");
  };

  return (
    <main className="container-xl h-auto">
      <div className="form-floating !h-75">
        <textarea
          className="form-control "
          placeholder="Leave a comment here"
          id="floatingTextarea"
          value={message}
          onChange={onChange}
        ></textarea>
        <label htmlFor="floatingTextarea">Coloque o texto desejado</label>
      </div>
      <button
        type="button"
        className="btn btn-primary btn-lg mt-4"
        onClick={send}
      >
        Enviar
      </button>
    </main>
  );
}
