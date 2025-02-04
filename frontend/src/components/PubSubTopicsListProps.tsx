import { PubSubTopic } from "@/types";

interface PubSubTopicsListProps {
  topics: PubSubTopic[];
}

export function PubSubTopicsList({ topics }: Readonly<PubSubTopicsListProps>) {
  return (
    <div className="bg-white shadow rounded-lg p-4">
      <div className="flex items-center mb-3">
        <h2 className="text-lg font-semibold">PubSub Topics</h2>
        <span className="ml-2 bg-purple-100 text-purple-800 text-xs font-medium px-2.5 py-0.5 rounded">
          {topics.length}
        </span>
      </div>
      <div className="grid gap-4">
        {topics.map((topic) => (
          <div key={topic.name} className="bg-gray-50 rounded-lg p-4">
            <div className="flex justify-between items-start">
              <h3 className="font-medium text-purple-600">{topic.name}</h3>
              <div className="text-sm">
                <span className="bg-purple-50 text-purple-700 px-2 py-0.5 rounded">
                  {topic.message_retention_duration}
                </span>
              </div>
            </div>

            {topic.kms_key_name && (
              <div className="mt-3 text-sm">
                <span className="text-gray-600">KMS Key:</span>
                <span className="ml-2 font-mono bg-gray-100 px-2 py-0.5 rounded">
                  {topic.kms_key_name}
                </span>
              </div>
            )}

            {topic.message_storage_policy && (
              <div className="mt-3">
                <span className="text-sm text-gray-600">Allowed Regions:</span>
                <div className="mt-1 flex flex-wrap gap-2">
                  {topic.message_storage_policy.allowed_persistence_regions.map(
                    (region) => (
                      <span
                        key={region}
                        className="text-xs bg-purple-50 text-purple-700 px-2 py-1 rounded-full"
                      >
                        {region}
                      </span>
                    )
                  )}
                </div>
              </div>
            )}

            {topic.schema_settings && (
              <div className="mt-3 grid grid-cols-2 gap-4 text-sm text-gray-600">
                <div>
                  Schema:{" "}
                  <span className="font-mono bg-gray-100 px-2 py-0.5 rounded">
                    {topic.schema_settings.schema}
                  </span>
                </div>
                <div>
                  Encoding:{" "}
                  <span className="font-mono bg-gray-100 px-2 py-0.5 rounded">
                    {topic.schema_settings.encoding}
                  </span>
                </div>
              </div>
            )}

            {topic.labels && Object.keys(topic.labels).length > 0 && (
              <div className="mt-3">
                <span className="text-sm text-gray-600">Labels:</span>
                <div className="mt-1 flex flex-wrap gap-2">
                  {Object.entries(topic.labels).map(([key, value]) => (
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
