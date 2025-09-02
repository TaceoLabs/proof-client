import React, { useState } from "react";
import { CoCircom, type ConfigurationParameters, JobApi, NodeApi, Configuration } from '@taceo/proof-client-bundler'

export default function CoCircomFullMultipleInputs() {
  const apiUrl = import.meta.env.PROD ? "https://proof.taceo.network" : "http://localhost:1234";
  const configParams: ConfigurationParameters = {
    basePath: apiUrl
  };
  const configuration = new Configuration(configParams)
  const jobInstance = new JobApi(configuration);
  const nodeInstance = new NodeApi(configuration);
  const [blueprint, setBlueprint] = useState<string>("");
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState<boolean>(false);
  const [jobId, setJobId] = useState<string | null>(null);
  const [jobNodes, setJobNodes] = useState<Array<number>>([]);
  const [deadline, setDeadline] = useState<Date | null>(null);
  const [voucher, setVoucher] = useState<string | null>(null);

  const handleScheduleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    setError(null);
    setLoading(true);

    try {
      const nodes = await nodeInstance.randomNodeProviders();
      setJobNodes([nodes.node0.id, nodes.node1.id, nodes.node2.id]);
      const jobId = await CoCircom.scheduleFullMultipleInputsJob(jobInstance, nodes, blueprint, voucher, deadline);
      setJobId(jobId);
      setLoading(false);
    } catch (error: any) {
      setError(error.message);
      setLoading(false);
    }
  };

  return (
    <div className="flex items-center justify-center py-10 px-15 md:rounded-[10pt] md:shadow-xl md:border md:border-current md:w-lg">
      <form className="w-full" onSubmit={handleScheduleSubmit}>
        <div className="grid gap-2">
          <h1 className="text-[40px] font-bold text-center">TACEO:Proof</h1>
          <div className="w-[5rem] h-[1rem] bg-[#52ffc5] mx-auto my-5"></div>
          <div>
            <h2 className="text-[14pt] font-bold pb-1">Blueprint</h2>
            <input required className="rounded-[5pt] shadow-xl border border-current p-2 w-full" type="text" onChange={(e) => setBlueprint(e.target.value)} />
          </div>
          <div>
            <h2 className="text-[14pt] font-bold pb-1">Voucher</h2>
            <input className="rounded-[5pt] shadow-xl border border-current p-2 w-full" type="text" onChange={(e) => setVoucher(e.target.value)} />
          </div>
          <div>
            <h2 className="text-[14pt] font-bold pb-1">Job Deadline</h2>
            <input className="rounded-[5pt] shadow-xl border border-current p-2 w-full" type="date" onChange={(e) => setDeadline(new Date(e.target.value))} />
          </div>
          <div className="pt-8 mx-auto">
            {loading ?
              <button className="text-[14pt] text-black font-bold rounded-[5pt] bg-[#52ffc5] py-2 pl-3 pr-5 inline-flex items-center" type="submit" disabled={true}>
                <svg className="mr-3 ml-1 size-6 animate-spin" viewBox="0 0 64 64">
                  <circle fill="none" strokeWidth="10" className="stroke-black opacity-40" cx="32" cy="32" r="24" />
                  <circle fill="none" strokeWidth="10" className="stroke-black" strokeDasharray="250" strokeDashoffset="210" cx="32" cy="32" r="24" />
                </svg>
                Loading...
              </button>
              :
              <button className="text-[14pt] text-black font-bold rounded-[5pt] bg-[#52ffc5] p-2 cursor-pointer px-12" type="submit">
                Schedule
              </button>
            }
          </div>
          <div className="pt-5 mx-auto text-center">
            {error && <div className="text-[#ff0000]">{error}</div>}
            {jobId && (
              <div>
                <p>Job ID: {jobId}</p>
                <p>Job Nodes: {jobNodes.join(",")}</p>
              </div>
            )}
          </div>
        </div>
      </form>
    </div>
  );
}
