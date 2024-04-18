"use client";

import getApiUrl from "@/helpers/api";
import { ChangeEvent, FormEvent, useEffect, useState } from "react";

interface MainPageProps {
  id?: string;
}

export default function MainPage({ id }: MainPageProps) {
  const [message, setMessage] = useState("");

  useEffect(() => {
    if (id) load();
  }, []);

  const onChange = (e: ChangeEvent<HTMLTextAreaElement>) => {
    setMessage(e.target.value);
  };

  const send = async (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();

    const objectSend = {
      message: message,
    };

    const result = await fetch(getApiUrl(), {
      method: "post",
      cache: "no-cache",
      headers: {
        "Content-Type": "application/json",
      },

      body: JSON.stringify(objectSend),
    });

    const response = await result.json();
    navigator.clipboard.writeText(`${location.origin}/${response.key}`);
    alert("Link copiado para clipbloard");
  };

  const load = async () => {
    const data = await fetch(getApiUrl() + "/" + id);
    const reponse = await data.json();
    setMessage(reponse.message);
  };

  const copyConetnt = () => {
    navigator.clipboard.writeText(message);
  };

  return (
    <main className="p-11">
      <form onSubmit={send}>
        <div className="w-full mb-4 border border-gray-200 rounded-lg bg-gray-50 dark:bg-gray-700 dark:border-gray-600">
          <div className="flex items-center justify-between px-3 py-2 border-b dark:border-gray-600">
            <button
              type="button"
              data-tooltip-target="tooltip-fullscreen"
              className="p-2 text-gray-500 rounded cursor-pointer sm:ms-auto hover:text-gray-900 hover:bg-gray-100 dark:text-gray-400 dark:hover:text-white dark:hover:bg-gray-600"
              onClick={copyConetnt}
            >
              <i className="fa-regular fa-copy"></i>
            </button>
          </div>
          <div className="px-4 py-2 bg-white rounded-b-lg dark:bg-gray-800">
            <label htmlFor="editor" className="sr-only">
              Publish post
            </label>
            <textarea
              id="editor"
              className="h-96 block w-full px-0 text-sm text-gray-800 bg-white border-0 dark:bg-gray-800 focus:ring-0 dark:text-white dark:placeholder-gray-400"
              placeholder="Write an article..."
              value={message}
              onChange={onChange}
              disabled={!!id}
              required
            />
          </div>
        </div>
        {!id && (
          <button
            type="submit"
            className="inline-flex items-center px-5 py-2.5 text-sm font-medium text-center text-white bg-blue-700 rounded-lg focus:ring-4 focus:ring-blue-200 dark:focus:ring-blue-900 hover:bg-blue-800"
          >
            Publish post
          </button>
        )}
      </form>
    </main>
  );
}
