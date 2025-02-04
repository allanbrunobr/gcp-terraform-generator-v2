"use client";

import { useState } from "react";
import { ServiceAccountCredentials } from "@/types";

interface FileUploadProps {
  onCredentialsLoad: (credentials: ServiceAccountCredentials) => void;
}

export function FileUpload({ onCredentialsLoad }: Readonly<FileUploadProps>) {
  const [error, setError] = useState<string>("");
  const [isDragging, setIsDragging] = useState(false);

  const validateCredentials = (
    credentials: any
  ): credentials is ServiceAccountCredentials => {
    if (
      typeof credentials.type !== "string" ||
      credentials.type !== "service_account"
    ) {
      setError("Invalid file: must be a service account");
      return false;
    }

    const requiredFields = [
      "project_id",
      "private_key_id",
      "private_key",
      "client_email",
      "client_id",
      "auth_uri",
      "token_uri",
    ];

    for (const field of requiredFields) {
      if (typeof credentials[field] !== "string" || !credentials[field]) {
        setError(`Required field missing: ${field}`);
        return false;
      }
    }

    if (!credentials.client_email.endsWith(".gserviceaccount.com")) {
      setError("Invalid email: must be a service account email");
      return false;
    }

    return true;
  };

  const handleFileChange = async (
    event: React.ChangeEvent<HTMLInputElement>
  ) => {
    const file = event.target.files?.[0];
    if (!file) return;

    if (!file.name.endsWith(".json")) {
      setError("Please select a .json file");
      return;
    }

    try {
      const text = await file.text();
      const credentials = JSON.parse(text);

      if (validateCredentials(credentials)) {
        setError("");
        onCredentialsLoad(credentials);
      }
    } catch (e) {
      setError("The file does not contain valid JSON");
    }
  };

  const handleDragOver = (e: React.DragEvent) => {
    e.preventDefault();
    setIsDragging(true);
  };

  const handleDragLeave = (e: React.DragEvent) => {
    e.preventDefault();
    setIsDragging(false);
  };

  const handleDrop = async (e: React.DragEvent) => {
    e.preventDefault();
    setIsDragging(false);

    const file = e.dataTransfer.files?.[0];
    if (!file) return;

    if (!file.name.endsWith(".json")) {
      setError("Please select a .json file");
      return;
    }

    try {
      const text = await file.text();
      const credentials = JSON.parse(text);

      if (validateCredentials(credentials)) {
        setError("");
        onCredentialsLoad(credentials);
      }
    } catch (e) {
      setError("The file does not contain valid JSON");
    }
  };

  return (
    <div className="space-y-4">
      <div
        className={`relative border-2 ${
          isDragging ? "border-blue-500" : "border-gray-300"
        } border-dashed rounded-xl p-8 transition-colors duration-200 ${
          isDragging ? "bg-blue-50" : "bg-white"
        } dark:bg-gray-800`}
        onDragOver={handleDragOver}
        onDragLeave={handleDragLeave}
        onDrop={handleDrop}
      >
        <div className="text-center space-y-4">
          <div className="text-gray-600 dark:text-gray-300">
            <p className="text-lg font-medium">
              Drop your service account credentials file here
            </p>
            <p className="text-sm mt-2">
              or click to select the file from your computer
            </p>
          </div>
          <input
            type="file"
            accept=".json"
            onChange={handleFileChange}
            className="block w-full text-sm text-gray-500 dark:text-gray-400
              file:mr-4 file:py-2 file:px-4
              file:rounded-full file:border-0
              file:text-sm file:font-semibold
              file:bg-blue-50 file:text-blue-700
              dark:file:bg-blue-900/50 dark:file:text-blue-300
              hover:file:bg-blue-100 dark:hover:file:bg-blue-900/70
              transition-colors cursor-pointer"
          />
        </div>
      </div>
      {error && (
        <div className="bg-red-50 dark:bg-red-900/30 border border-red-200 dark:border-red-800 text-red-700 dark:text-red-200 px-4 py-3 rounded-lg text-sm">
          {error}
        </div>
      )}
    </div>
  );
}
