interface TerraformCodePreviewProps {
  code: string;
  onCopy: () => void;
}

export function TerraformCodePreview({
  code,
  onCopy,
}: TerraformCodePreviewProps) {
  return (
    <div className="mt-4">
      <div className="flex justify-between items-center mb-2">
        <h3 className="text-lg font-semibold">Código Terraform Gerado</h3>
        <button
          onClick={onCopy}
          className="bg-blue-500 text-white px-4 py-2 rounded hover:bg-blue-600 transition-colors flex items-center gap-2"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            className="h-4 w-4"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"
            />
          </svg>
          Copiar Código
        </button>
      </div>
      <div className="bg-gray-800 text-white p-4 rounded-lg">
        <pre className="overflow-x-auto font-mono text-sm">
          <code className="language-hcl">{code}</code>
        </pre>
      </div>
    </div>
  );
}
