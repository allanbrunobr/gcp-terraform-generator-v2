// src/components/CredentialsPreview.tsx
"use client";
import React, { Fragment, useState } from "react";
import { ServiceAccountCredentials } from "@/types";
import { Dialog, Transition } from "@headlessui/react";

interface CredentialsPreviewProps {
  credentials: Readonly<ServiceAccountCredentials>;
  onConfirm: () => void;
}

export function CredentialsPreview({
  credentials,
  onConfirm,
}: Readonly<CredentialsPreviewProps>) {
  const [isOpen, setIsOpen] = useState(true);

  const maskSensitiveValue = (key: string, value: string) => {
    const sensitiveFields = ["private_key", "private_key_id", "client_id"];
    return sensitiveFields.includes(key) ? "************" : value;
  };

  return (
    <>
      <Transition appear show={isOpen} as={Fragment}>
        <Dialog
          as="div"
          className="fixed inset-0 z-10 overflow-y-auto"
          onClose={() => setIsOpen(false)}
        >
          <div className="min-h-screen px-4 text-center">
            <Transition
              show={isOpen}
              as={Fragment}
              enter="ease-out duration-300"
              enterFrom="opacity-0"
              enterTo="opacity-50"
              leave="ease-in duration-200"
              leaveFrom="opacity-50"
              leaveTo="opacity-0"
            >
              <div className="fixed inset-0 bg-black opacity-50" />
            </Transition>

            {/* Browser centering trick */}
            <span className="inline-block h-screen align-middle" aria-hidden="true">
              &#8203;
            </span>
            <Transition
              show={isOpen}
              as={Fragment}
              enter="ease-out duration-300"
              enterFrom="opacity-0 scale-95"
              enterTo="opacity-100 scale-100"
              leave="ease-in duration-200"
              leaveFrom="opacity-100 scale-100"
              leaveTo="opacity-0 scale-95"
            >
              <div className="inline-block w-full max-w-md p-6 my-8 overflow-hidden text-left align-middle transition-all transform bg-gradient-to-r from-blue-100 via-blue-200 to-blue-300 shadow-xl rounded-2xl">
                <Dialog.Title
                  as="h3"
                  className="text-lg font-medium leading-6 text-blue-800 mb-4"
                >
                  Credenciais Carregadas
                </Dialog.Title>
                <div className="font-mono text-sm space-y-2 overflow-y-auto max-h-60">
                  {(Object.entries(credentials) as [string, string][]).map(
                    ([key, value]) => (
                      <div key={key} className="flex justify-between">
                        <span className="text-blue-700 font-medium capitalize">
                          {key.replace(/_/g, " ")}:
                        </span>
                        <span className="text-gray-700">
                          {maskSensitiveValue(key, value)}
                        </span>
                      </div>
                    )
                  )}
                </div>

                <div className="mt-6">
                  <button
                    type="button"
                    className="inline-flex justify-center px-4 py-2 text-sm font-medium text-white bg-blue-600 border border-transparent rounded-md hover:bg-blue-700 focus:outline-none focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:ring-blue-500"
                    onClick={() => {
                      setIsOpen(false);
                      onConfirm();
                    }}
                  >
                    Conectar ao GCloud e Gerar Terraform
                  </button>
                </div>
              </div>
            </Transition>
          </div>
        </Dialog>
      </Transition>
    </>
  );
}
