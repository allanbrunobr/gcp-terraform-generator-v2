// src/components/PubSubSubscriptionsList.tsx
import { PubSubSubscription } from "@/types";

interface PubSubSubscriptionsListProps {
  subscriptions: PubSubSubscription[];
}

export function PubSubSubscriptionsList({
  subscriptions,
}: Readonly<PubSubSubscriptionsListProps>) {
  return (
    <div className="bg-white shadow rounded-lg p-4">
      <div className="flex items-center mb-3">
        <h2 className="text-lg font-semibold">PubSub Subscriptions</h2>
        <span className="ml-2 bg-indigo-100 text-indigo-800 text-xs font-medium px-2.5 py-0.5 rounded">
          {subscriptions.length}
        </span>
      </div>
      <div className="grid gap-4">
        {subscriptions.map((subscription) => (
          <div key={subscription.name} className="bg-gray-50 rounded-lg p-4">
            <div className="flex justify-between items-start">
              <h3 className="font-medium text-indigo-600">
                {subscription.name}
              </h3>
              <div className="text-sm">
                <span className="bg-gray-100 text-gray-700 px-2 py-0.5 rounded">
                  Topic: {subscription.topic}
                </span>
              </div>
            </div>

            <div className="mt-3 grid grid-cols-2 gap-4 text-sm text-gray-600">
              <div>
                Ack Deadline:{" "}
                <span className="font-mono bg-gray-100 px-2 py-0.5 rounded">
                  {subscription.ack_deadline_seconds}s
                </span>
              </div>
              <div>
                Retention:{" "}
                <span className="font-mono bg-gray-100 px-2 py-0.5 rounded">
                  {subscription.message_retention_duration}
                </span>
              </div>
            </div>

            {subscription.push_config && (
              <div className="mt-3">
                <span className="text-sm text-gray-600">Push Endpoint:</span>
                <div className="mt-1">
                  <code className="text-xs bg-gray-100 px-2 py-1 rounded break-all">
                    {subscription.push_config.push_endpoint}
                  </code>
                </div>

                {subscription.push_config.attributes &&
                  Object.keys(subscription.push_config.attributes).length >
                    0 && (
                    <div className="mt-2">
                      <span className="text-sm text-gray-600">
                        Push Attributes:
                      </span>
                      <div className="mt-1 flex flex-wrap gap-2">
                        {Object.entries(
                          subscription.push_config.attributes
                        ).map(([key, value]) => (
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
            )}

            {subscription.filter && (
              <div className="mt-3">
                <span className="text-sm text-gray-600">Filter:</span>
                <div className="mt-1">
                  <code className="text-xs bg-gray-100 px-2 py-1 rounded">
                    {subscription.filter}
                  </code>
                </div>
              </div>
            )}

            <div className="mt-3 text-sm">
              <div className="flex items-center gap-4">
                <span
                  className={`inline-flex items-center px-2 py-1 rounded-full text-xs font-medium ${
                    subscription.retain_acked_messages
                      ? "bg-green-100 text-green-800"
                      : "bg-gray-100 text-gray-800"
                  }`}
                >
                  {subscription.retain_acked_messages
                    ? "Retains Acked Messages"
                    : "No Message Retention"}
                </span>
                <span
                  className={`inline-flex items-center px-2 py-1 rounded-full text-xs font-medium ${
                    subscription.enable_message_ordering
                      ? "bg-blue-100 text-blue-800"
                      : "bg-gray-100 text-gray-800"
                  }`}
                >
                  {subscription.enable_message_ordering
                    ? "Ordered Messages"
                    : "Unordered Messages"}
                </span>
              </div>
            </div>

            {subscription.dead_letter_policy && (
              <div className="mt-3">
                <span className="text-sm text-gray-600">Dead Letter:</span>
                <div className="mt-1 text-xs">
                  <div className="bg-gray-100 px-2 py-1 rounded">
                    Topic: {subscription.dead_letter_policy.dead_letter_topic}
                    <br />
                    Max Attempts:{" "}
                    {subscription.dead_letter_policy.max_delivery_attempts}
                  </div>
                </div>
              </div>
            )}
          </div>
        ))}
      </div>
    </div>
  );
}
