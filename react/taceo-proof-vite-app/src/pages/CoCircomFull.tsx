import React, { type ChangeEvent, useRef, useState } from "react";
import { type ConfigurationParameters, JobApi, NodeApi, Configuration, BlueprintCurve, CoCircom, type Groth16Proof, type PublicSignals } from '@taceo/proof-client-bundler'

export default function CoCircomFull() {
  const apiUrl = import.meta.env.PROD ? "https://proof.taceo.network" : "http://localhost:1234";
  const websocketUrl = apiUrl.replace(/^https:/, 'wss:').replace(/^http:/, 'ws:') + "/api/v1/reports/subs";
  const configParams: ConfigurationParameters = {
    basePath: apiUrl
  };
  const configuration = new Configuration(configParams)
  const jobInstance = new JobApi(configuration);
  const nodeInstance = new NodeApi(configuration);
  const [blueprint, setBlueprint] = useState<string>("");
  const [curve, setCurve] = useState<BlueprintCurve>(BlueprintCurve.Bn254);
  const [error, setError] = useState<string | null>(null);
  const fileRef = useRef<HTMLInputElement>(null);
  const [file, setFile] = useState<File | null>(null);
  const [loading, setLoading] = useState<boolean>(false);
  const [proof, setProof] = useState<Groth16Proof | null>(null);
  const [publicInputsOut, setPublicInputsOut] = useState<PublicSignals | null>(null);
  const [publicInputs, setPublicInputs] = useState<Array<string>>([]);
  const [voucher, setVoucher] = useState<string | null>(null);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    setError(null);
    setProof(null);
    setPublicInputsOut(null);
    setLoading(true);

    try {
      const nodes = await nodeInstance.randomNodeProviders();
      const input = JSON.parse(await file!.text());
      const jobId = await CoCircom.scheduleFullJob(jobInstance, nodes, blueprint, voucher, curve, publicInputs, input);
      const jobResult = await CoCircom.fetchJobResult(websocketUrl, jobId);
      setProof(jobResult.proof);
      setPublicInputsOut(jobResult.public_inputs);
      setLoading(false);
    } catch (error: any) {
      setError(error.message);
      setLoading(false);
    }
  };

  const handleFileOnChange = (e: ChangeEvent<HTMLInputElement>) => {
    const files = e.target.files;
    if (files && files.length > 0) {
      setFile(files[0]);
    }
  };

  const handleFileOnClick = () => {
    fileRef.current?.click();
  };

  return (
    <div className="flex items-center justify-center py-10 px-15 md:rounded-[10pt] md:shadow-xl md:border md:border-current md:w-lg">
      <form className="w-full" onSubmit={handleSubmit}>
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
            <h2 className="text-[14pt] font-bold pb-1">Curve</h2>
            <select required className="rounded-[5pt] shadow-xl border border-current p-2 w-full" onChange={(e) => setCurve(e.target.value as BlueprintCurve)}>
              <option value='Bn254'>BN254</option>
              <option value='Bls381'>BLS12_381</option>
            </select>
          </div>
          <div>
            <h2 className="text-[14pt] font-bold pb-1">Public Inputs</h2>
            <input className="rounded-[5pt] shadow-xl border border-current p-2 w-full" type="text" onChange={(e) => setPublicInputs(e.target.value.split(','))} />
          </div>
          <div>
            <h2 className="text-[14pt] font-bold pb-1">Input</h2>
            <input required type="file" ref={fileRef} onChange={handleFileOnChange} style={{ display: 'none' }} />
            <button className="rounded-[5pt] shadow-xl border border-current p-2 w-full cursor-pointer" onClick={handleFileOnClick} type="button">
              {file ? file.name : 'Choose File'}
            </button>
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
                Submit
              </button>
            }
          </div>
          <div className="pt-5 mx-auto text-center">
            {error && <div className="text-[#ff0000]">{error}</div>}
            {proof && (
              <div>
                <a className="underline text-current" href={`data:text/json;charset=utf-8,${encodeURIComponent(JSON.stringify(proof))}`} download="proof.json">
                  Download Proof
                </a>
                <br />
                <a className="underline text-current" href={`data:text/json;charset=utf-8,${encodeURIComponent(JSON.stringify(publicInputsOut!))}`} download="public.json">
                  Download Public Inputs
                </a>
              </div>
            )}
          </div>
        </div>
      </form>
    </div>
  );
}
