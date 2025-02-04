// src/components/DestinationCredentialsModal.tsx
import { useState } from "react";
import { ServiceAccountCredentials } from "@/types";
import { FileUpload } from "./FileUpload";

interface DestinationCredentialsModalProps {
  isOpen: boolean;
  onClose: () => void;
  onConfirm: (credentials: ServiceAccountCredentials) => void;
}

export function DestinationCredentialsModal({
  isOpen,
  onClose,
  onConfirm,
}: DestinationCredentialsModalProps) {
  const [credentials, setCredentials] =
    useState<ServiceAccountCredentials | null>(null);

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4">
      <div className="bg-white rounded-lg p-6 w-full max-w-2xl">
        <div className="flex justify-between items-center mb-4">
          <h2 className="text-xl font-semibold">
            Credenciais do Projeto Destino
          </h2>
          <button
            onClick={onClose}
            className="text-gray-500 hover:text-gray-700"
          >
            <svg
              className="w-6 h-6"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M6 18L18 6M6 6l12 12"
              />
            </svg>
          </button>
        </div>

        <div className="mb-6">
          <p className="text-gray-600 mb-4">
            Forneça as credenciais do projeto GCP para onde a infraestrutura
            será migrada.
          </p>
          <FileUpload
            onCredentialsLoad={(creds) => {
              setCredentials(creds);
            }}
          />
        </div>

        <div className="flex justify-end gap-4">
          <button
            onClick={onClose}
            className="px-4 py-2 border border-gray-300 rounded-lg hover:bg-gray-50"
          >
            Cancelar
          </button>
          <button
            onClick={() => credentials && onConfirm(credentials)}
            disabled={!credentials}
            className="px-4 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            Iniciar Migração
          </button>
        </div>
      </div>
    </div>
  );
}
