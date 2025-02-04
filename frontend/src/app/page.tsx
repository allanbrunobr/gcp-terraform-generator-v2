// src/app/page.tsx
"use client";

import { useState } from "react";
import { FileUpload } from "@/components/FileUpload";
import { CredentialsPreview } from "@/components/CredentialsPreview";
import {
  ServiceAccountCredentials,
  GCPResource,
  CloudFunction,
  CloudRun,
  GCSBucket,
  PubSubTopic,
  PubSubSubscription,
  BigQueryDataset,
  TerraformResponse,
} from "@/types";
import { CloudFunctionsList } from "@/components/CloudFunctionsList";
import { TerraformCodePreview } from "@/components/TerraformCodePreview";
import { CloudRunsList } from "@/components/CloudRunsList";
import { VPCList } from "@/components/VPCList";
import { StorageBucketsList } from "@/components/StorageBucketsList";
import { PubSubTopicsList } from "@/components/PubSubTopicsListProps";
import { PubSubSubscriptionsList } from "@/components/PubSubSubscriptionsList";
import { BigQueryDatasetsList } from "@/components/BigQueryDatasetsListProps";
import { DestinationCredentialsModal } from "@/components/DestinationCredentialsModal";

export default function Home() {
  const [credentials, setCredentials] =
    useState<ServiceAccountCredentials | null>(null);
  const [terraformCode, setTerraformCode] = useState("");
  const [error, setError] = useState("");
  const [resources, setResources] = useState<GCPResource[]>([]);
  const [cloudFunctions, setCloudFunctions] = useState<CloudFunction[]>([]);
  const [cloudRuns, setCloudRuns] = useState<CloudRun[]>([]);
  const [buckets, setBuckets] = useState<GCSBucket[]>([]);
  const [pubsubTopics, setPubsubTopics] = useState<PubSubTopic[]>([]);
  const [pubsubSubscriptions, setPubsubSubscriptions] = useState<
    PubSubSubscription[]
  >([]);
  const [datasets, setDatasets] = useState<BigQueryDataset[]>([]);
  const [isDestinationModalOpen, setIsDestinationModalOpen] = useState(false);
  const [destinationCredentials, setDestinationCredentials] =
    useState<ServiceAccountCredentials | null>(null);
  const [isMigrating, setIsMigrating] = useState(false);
  const [success, setSuccess] = useState<string>("");

  const handleCredentialsLoad = (creds: ServiceAccountCredentials) => {
    setCredentials(creds);
    setError("");
  };

  const handleGenerateCode = async () => {
    if (!credentials) return;

    try {
      const response = await fetch("http://localhost:3001/generate", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(credentials),
      });

      if (!response.ok) {
        const errorText = await response.text();
        console.error("3. Erro recebido:", errorText);
        throw new Error(errorText);
      }

      const data = await response.json();

      setTerraformCode(data.terraform_code);
      setResources(data.resources);
      setCloudFunctions(data.cloud_functions);
      setCloudRuns(data.cloud_runs);
      setBuckets(data.storage_buckets);
      setPubsubTopics(data.pubsub_topics);
      setPubsubSubscriptions(data.pubsub_subscriptions);
      setDatasets(data.bigquery_datasets);
    } catch (error) {
      console.error("ERRO:", error);
      setError(
        error instanceof Error ? error.message : "Erro ao processar arquivo"
      );
      setTerraformCode("");
      setResources([]);
      setCloudFunctions([]);
      setCloudRuns([]);
      setBuckets([]);
      setPubsubTopics([]);
      setPubsubSubscriptions([]);
      setDatasets([]);
    }
  };

  const handleStartMigration = async (
    destCredentials: ServiceAccountCredentials
  ) => {
    setIsMigrating(true);
    try {
      const response = await fetch("http://localhost:3001/migrate", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          source_credentials: credentials,
          destination_credentials: destCredentials,
          terraform_code: terraformCode,
        }),
      });

      if (!response.ok) {
        throw new Error(await response.text());
      }

      const result = await response.json();
      // Mostrar mensagem de sucesso
      setSuccess(
        `Arquivos Terraform gerados em: ${result.terraform_files_path}`
      );
    } catch (error) {
      console.error("Erro na migração:", error);
      setError(
        error instanceof Error ? error.message : "Erro ao executar migração"
      );
    } finally {
      setIsMigrating(false);
      setIsDestinationModalOpen(false);
    }
  };

  // Função para verificar se tem recursos para migrar
  const hasResourcesToMigrate = () => {
    return (
      resources.length > 0 ||
      cloudFunctions.length > 0 ||
      cloudRuns.length > 0 ||
      buckets.length > 0 ||
      pubsubTopics.length > 0 ||
      pubsubSubscriptions.length > 0 ||
      datasets.length > 0
    );
  };

  return (
    <main className="container mx-auto p-4 max-w-4xl">
      <h1 className="text-2xl font-bold mb-6">GCP Terraform Generator</h1>

      <div className="space-y-6">
        <FileUpload onCredentialsLoad={handleCredentialsLoad} />
        {/* Mensagem de Erro */}
        {error && (
          <div className="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded">
            {error}
          </div>
        )}

        {/* Mensagem de Sucesso */}
        {success && (
          <div className="bg-green-100 border border-green-400 text-green-700 px-4 py-3 rounded">
            {success}
          </div>
        )}
        {credentials && (
          <CredentialsPreview
            credentials={credentials}
            onConfirm={handleGenerateCode}
          />
        )}

        {resources.length > 0 && <VPCList resources={resources} />}

        {cloudFunctions.length > 0 && (
          <CloudFunctionsList functions={cloudFunctions} />
        )}

        {cloudRuns && cloudRuns.length > 0 && (
          <CloudRunsList services={cloudRuns} />
        )}

        {buckets.length > 0 && <StorageBucketsList buckets={buckets} />}

        {pubsubTopics.length > 0 && <PubSubTopicsList topics={pubsubTopics} />}

        {pubsubSubscriptions.length > 0 && (
          <PubSubSubscriptionsList subscriptions={pubsubSubscriptions} />
        )}

        {datasets.length > 0 && <BigQueryDatasetsList datasets={datasets} />}

        {terraformCode && (
          <TerraformCodePreview
            code={terraformCode}
            onCopy={() => navigator.clipboard.writeText(terraformCode)}
          />
        )}
        {/* Botão de Migração - Aparece apenas quando há recursos listados */}
        {hasResourcesToMigrate() && (
          <div className="fixed bottom-8 right-8">
            <button
              onClick={() => setIsDestinationModalOpen(true)}
              disabled={isMigrating}
              className="bg-green-500 hover:bg-green-600 text-white px-6 py-3 rounded-lg shadow-lg flex items-center gap-2 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              {isMigrating ? (
                <>
                  <svg className="animate-spin h-5 w-5" viewBox="0 0 24 24">
                    <circle
                      className="opacity-25"
                      cx="12"
                      cy="12"
                      r="10"
                      stroke="currentColor"
                      strokeWidth="4"
                      fill="none"
                    />
                    <path
                      className="opacity-75"
                      fill="currentColor"
                      d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"
                    />
                  </svg>
                  Migrando...
                </>
              ) : (
                <>
                  <svg
                    className="w-5 h-5"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                  >
                    <path
                      strokeLinecap="round"
                      strokeLinejoin="round"
                      strokeWidth={2}
                      d="M8 7h12m0 0l-4-4m4 4l-4 4m0 6H4m0 0l4 4m-4-4l4-4"
                    />
                  </svg>
                  Iniciar Migração
                </>
              )}
            </button>
          </div>
        )}

        {/* Modal de Credenciais do Destino */}
        <DestinationCredentialsModal
          isOpen={isDestinationModalOpen}
          onClose={() => setIsDestinationModalOpen(false)}
          onConfirm={handleStartMigration}
        />
      </div>
    </main>
  );
}
