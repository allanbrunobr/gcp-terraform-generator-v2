import { GCPResource } from "@/types";

interface VPCListProps {
  resources: GCPResource[];
}

export function VPCList({ resources }: VPCListProps) {
  return (
    <div className="bg-white shadow rounded-lg p-4">
      <div className="flex items-center mb-3">
        <h2 className="text-lg font-semibold">VPCs</h2>
        <span className="ml-2 bg-blue-100 text-blue-800 text-xs font-medium px-2.5 py-0.5 rounded">
          {resources.length}
        </span>
      </div>
      <div className="grid gap-4">
        {resources.map((resource) => (
          <div
            key={`${resource.name}-${resource.resource_type}`}
            className="bg-gray-50 rounded-lg p-4"
          >
            <h3 className="font-medium text-blue-600">{resource.name}</h3>
            <div className="mt-2 text-sm text-gray-600">
              <p>Tipo: {resource.resource_type}</p>
              <p>
                Auto Create Subnets:{" "}
                {resource.details.autoCreateSubnetworks ? "Yes" : "No"}
              </p>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
