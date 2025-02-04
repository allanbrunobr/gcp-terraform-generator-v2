"use client";

interface CodePreviewProps {
  code: string;
}

export function CodePreview({ code }: Readonly<CodePreviewProps>) {
  return (
    <div className="relative">
      <pre className="bg-gray-800 text-white p-4 rounded overflow-x-auto">
        <code>{code}</code>
      </pre>
      <button
        onClick={() => navigator.clipboard.writeText(code)}
        className="absolute top-2 right-2 bg-gray-700 text-white px-2 py-1 rounded text-sm"
      >
        Copiar
      </button>
    </div>
  );
}
