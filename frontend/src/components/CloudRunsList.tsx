import { CloudRun } from "@/types";

interface CloudRunsListProps {
  services: CloudRun[];
}

export function CloudRunsList({ services }: CloudRunsListProps) {
  return (
    <div className="bg-white shadow rounded-lg p-4">
      <div className="flex items-center mb-3">
        <h2 className="text-lg font-semibold">Cloud Run Services</h2>
        <span className="ml-2 bg-green-100 text-green-800 text-xs font-medium px-2.5 py-0.5 rounded">
          {services.length}
        </span>
      </div>
      <div className="grid gap-4">
        {services.map((service, index) => (
          <div key={index} className="bg-gray-50 rounded-lg p-4">
            <div className="flex justify-between items-start">
              <h3 className="font-medium text-green-600">{service.name}</h3>
              <span className="text-sm text-gray-500">{service.location}</span>
            </div>
            <div className="mt-3 grid grid-cols-2 gap-4 text-sm text-gray-600">
              <div>
                CPU:{" "}
                <span className="font-mono bg-gray-100 px-2 py-0.5 rounded">
                  {service.cpu}
                </span>
              </div>
              <div>
                Memory:{" "}
                <span className="font-mono bg-gray-100 px-2 py-0.5 rounded">
                  {service.memory}
                </span>
              </div>
              <div>
                Min Instances:{" "}
                <span className="font-mono bg-gray-100 px-2 py-0.5 rounded">
                  {service.min_scale || "0"}
                </span>
              </div>
              <div>
                Max Instances:{" "}
                <span className="font-mono bg-gray-100 px-2 py-0.5 rounded">
                  {service.max_scale || "Auto"}
                </span>
              </div>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
