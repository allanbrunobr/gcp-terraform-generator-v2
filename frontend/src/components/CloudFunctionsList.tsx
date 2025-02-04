// src/components/CloudFunctionsList.tsx
import { CloudFunction } from "@/types";

interface CloudFunctionsListProps {
  functions: CloudFunction[];
}

export function CloudFunctionsList({
  functions,
}: Readonly<CloudFunctionsListProps>) {
  return (
    <div className="bg-white shadow rounded-lg p-4">
      <h2 className="text-lg font-semibold mb-3">Cloud Functions</h2>
      <div className="grid gap-4">
        {functions.map((func) => (
          <div key={func.name} className="bg-gray-50 rounded-lg p-4">
            <div className="flex justify-between items-start">
              <div>
                <h3 className="font-medium text-lg text-blue-600">
                  {func.name}
                </h3>
                <span
                  className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${
                    func.status === "ACTIVE"
                      ? "bg-green-100 text-green-800"
                      : "bg-yellow-100 text-yellow-800"
                  } mt-1`}
                >
                  {func.status}
                </span>
              </div>
              <span className="text-sm text-gray-500">
                Region: {func.region}
              </span>
            </div>

            <div className="mt-3 grid grid-cols-2 gap-4">
              <div className="text-sm">
                <span className="text-gray-500">Runtime:</span>
                <span className="ml-2 font-mono bg-gray-100 px-2 py-1 rounded">
                  {func.runtime}
                </span>
              </div>
              <div className="text-sm">
                <span className="text-gray-500">Entry Point:</span>
                <span className="ml-2 font-mono bg-gray-100 px-2 py-1 rounded">
                  {func.entry_point}
                </span>
              </div>
            </div>

            <div className="mt-3 text-sm">
              <span className="text-gray-500">Project ID:</span>
              <span className="ml-2 font-mono bg-gray-100 px-2 py-1 rounded">
                {func.project_id}
              </span>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
