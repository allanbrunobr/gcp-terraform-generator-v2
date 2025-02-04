import { GCSBucket } from "@/types";

interface StorageBucketsListProps {
  buckets: GCSBucket[];
}

export function StorageBucketsList({
  buckets,
}: Readonly<StorageBucketsListProps>) {
  return (
    <div className="bg-white shadow rounded-lg p-4">
      <div className="flex items-center mb-3">
        <h2 className="text-lg font-semibold">Storage Buckets</h2>
        <span className="ml-2 bg-yellow-100 text-yellow-800 text-xs font-medium px-2.5 py-0.5 rounded">
          {buckets.length}
        </span>
      </div>
      <div className="grid gap-4">
        {buckets.map((bucket) => (
          <div key={bucket.name} className="bg-gray-50 rounded-lg p-4">
            <div className="flex justify-between items-start">
              <h3 className="font-medium text-yellow-600">{bucket.name}</h3>
              <span className="text-sm text-gray-500">{bucket.location}</span>
            </div>

            <div className="mt-3 grid grid-cols-2 gap-4 text-sm text-gray-600">
              <div>
                Storage Class:{" "}
                <span className="font-mono bg-gray-100 px-2 py-0.5 rounded">
                  {bucket.storage_class}
                </span>
              </div>
              <div>
                Versioning:{" "}
                <span className="font-mono bg-gray-100 px-2 py-0.5 rounded">
                  {bucket.versioning?.enabled ? "Enabled" : "Disabled"}
                </span>
              </div>
            </div>

            {bucket.lifecycle_rule && bucket.lifecycle_rule.length > 0 && (
              <div className="mt-3">
                <h4 className="text-sm font-medium text-gray-600 mb-2">
                  Lifecycle Rules
                </h4>
                <div className="space-y-2">
                  {bucket.lifecycle_rule.map((rule, index) => (
                    <div
                      key={index}
                      className="text-sm bg-gray-100 p-2 rounded"
                    >
                      <span className="font-medium">{rule.action.type}</span>
                      {rule.action.storage_class && (
                        <span className="ml-2">
                          → {rule.action.storage_class}
                        </span>
                      )}
                    </div>
                  ))}
                </div>
              </div>
            )}

            {bucket.labels && Object.keys(bucket.labels).length > 0 && (
              <div className="mt-3">
                <h4 className="text-sm font-medium text-gray-600 mb-2">
                  Labels
                </h4>
                <div className="flex flex-wrap gap-2">
                  {Object.entries(bucket.labels).map(([key, value]) => (
                    <span
                      key={key}
                      className="text-xs bg-gray-200 px-2 py-1 rounded"
                    >
                      {key}: {value}
                    </span>
                  ))}
                </div>
              </div>
            )}
          </div>
        ))}
      </div>
    </div>
  );
}
