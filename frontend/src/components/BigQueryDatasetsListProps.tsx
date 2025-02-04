import { BigQueryDataset } from "@/types";

interface BigQueryDatasetsListProps {
  datasets: BigQueryDataset[];
}

export function BigQueryDatasetsList({
  datasets,
}: Readonly<BigQueryDatasetsListProps>) {
  return (
    <div className="bg-white shadow rounded-lg p-4">
      <div className="flex items-center mb-3">
        <h2 className="text-lg font-semibold">BigQuery Datasets</h2>
        <span className="ml-2 bg-green-100 text-green-800 text-xs font-medium px-2.5 py-0.5 rounded">
          {datasets.length}
        </span>
      </div>
      <div className="grid gap-4">
        {datasets.map((dataset) => (
          <div key={dataset.dataset_id} className="bg-gray-50 rounded-lg p-4">
            <div className="flex justify-between items-start">
              <div>
                <h3 className="font-medium text-green-600">
                  {dataset.dataset_id}
                </h3>
                {dataset.friendly_name && (
                  <p className="text-sm text-gray-500 mt-1">
                    {dataset.friendly_name}
                  </p>
                )}
              </div>
              <span className="text-sm bg-green-50 text-green-700 px-2 py-0.5 rounded">
                {dataset.location}
              </span>
            </div>

            {dataset.description && (
              <div className="mt-3 text-sm text-gray-600">
                <p>{dataset.description}</p>
              </div>
            )}

            {dataset.default_table_expiration_ms && (
              <div className="mt-3 text-sm">
                <span className="text-gray-600">Default Table Expiration:</span>
                <span className="ml-2 font-mono bg-gray-100 px-2 py-0.5 rounded">
                  {Math.floor(
                    dataset.default_table_expiration_ms / (1000 * 60 * 60 * 24)
                  )}{" "}
                  days
                </span>
              </div>
            )}

            {dataset.access && dataset.access.length > 0 && (
              <div className="mt-3">
                <h4 className="text-sm font-medium text-gray-600 mb-2">
                  Access Control
                </h4>
                <div className="space-y-2">
                  {dataset.access.map((access, index) => (
                    <div
                      key={index}
                      className="text-sm bg-gray-100 p-2 rounded flex items-center justify-between"
                    >
                      <span className="font-medium">{access.role}</span>
                      <span className="text-gray-600">
                        {access.user_by_email ||
                          access.group_by_email ||
                          access.domain ||
                          access.special_group}
                      </span>
                    </div>
                  ))}
                </div>
              </div>
            )}

            {dataset.encryption_configuration && (
              <div className="mt-3 text-sm">
                <span className="text-gray-600">Encryption Key:</span>
                <span className="ml-2 font-mono bg-gray-100 px-2 py-0.5 rounded">
                  {dataset.encryption_configuration.kms_key_name}
                </span>
              </div>
            )}

            {dataset.labels && Object.keys(dataset.labels).length > 0 && (
              <div className="mt-3">
                <h4 className="text-sm font-medium text-gray-600 mb-2">
                  Labels
                </h4>
                <div className="flex flex-wrap gap-2">
                  {Object.entries(dataset.labels).map(([key, value]) => (
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
