import React, { ChangeEvent, useRef, useState } from "react";
import { BlueprintCurve, Configuration, ConfigurationParameters, JobApi, JobStatus, JobType, ProofResult } from '@taceo/proof-api-client';
import { scheduleFullJobRep3, scheduleProveJobShamir, scheduleProveJobRep3 } from '@taceo/proof-client-browser'
import wc from "../witness-calculator.js"; // generated with circom

const configParams: ConfigurationParameters = {
  basePath: "http://localhost:1234",
}
const congiuration = new Configuration(configParams)
const apiInstance = new JobApi(congiuration);

export default function Home() {
  const [code, setCode] = useState<string | null>(null);
  const [blueprint, setBlueprint] = useState<string | null>(null);
  const [curve, setCurve] = useState<BlueprintCurve>(BlueprintCurve.Bn254);
  const [result, setResult] = useState<ProofResult | null>(null);
  const [loading, setLoading] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);
  const [jobType, setJobType] = useState<JobType>(JobType.ShamirProve);
  const [numInputs, setNumInputs] = useState<number | null>(null);
  const [publicInputs, setPublicInputs] = useState<Array<string>>([]);
  const [selectedFile, setSelectedFile] = useState<File | null>(null);
  const fileInputRef = useRef<HTMLInputElement>(null);
  const [wasm, setWasm] = useState<File | null>(null);
  const wasmRef = useRef<HTMLInputElement>(null);

  const pollProofResult = async (id: string): Promise<ProofResult | null> => {
    while (true) {
      try {
        const getResultRes = await apiInstance.getResult({ id: id });
        if (getResultRes.status == JobStatus.Success) {
          return getResultRes.ok!;
        } else if (getResultRes.status == JobStatus.Failed) {
          setError(getResultRes.error ?? "something went wrong");
          return null;
        }
      } catch (error) {
        console.error('error:', error);
        return null;
      }
      await new Promise(resolve => setTimeout(resolve, 1000));
    }
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    setError(null);
    setResult(null);

    if (selectedFile == null) {
      setError("input file missing");
      return;
    }

    if (wasm == null && jobType != JobType.Rep3Full) {
      setError("wasm file missing");
      return;
    }

    const input = JSON.parse(await selectedFile!.text());
    let jobId;
    let witnessCalculator;
    let witness;

    setLoading(true);

    try {
      switch (jobType) {
        case JobType.Rep3Full:
          jobId = await scheduleFullJobRep3(apiInstance, code!, blueprint!, curve, publicInputs!, input);
          break;
        case JobType.Rep3Prove:
          witnessCalculator = await wc(await wasm!.bytes());
          witness = await witnessCalculator.calculateWTNSBin(input, 0);
          jobId = await scheduleProveJobRep3(apiInstance, code!, blueprint!, curve, numInputs!, witness);
          break;
        case JobType.ShamirProve:
          witnessCalculator = await wc(await wasm!.bytes());
          witness = await witnessCalculator.calculateWTNSBin(input, 0);
          jobId = await scheduleProveJobShamir(apiInstance, code!, blueprint!, curve, numInputs!, witness);
          break;

      }
      const result = await pollProofResult(jobId!);
      setResult(result);
    } catch (error: any) {
      setError(error.message);
    } finally {
      setLoading(false);
    }
  };

  const handleInputFileChange = (e: ChangeEvent<HTMLInputElement>) => {
    const files = e.target.files;
    if (files && files.length > 0) {
      setSelectedFile(files[0]);
    }
  };

  const handleWasmFileChange = (e: ChangeEvent<HTMLInputElement>) => {
    const files = e.target.files;
    if (files && files.length > 0) {
      setWasm(files[0]);
    }
  };

  const handleInputUploadClick = () => {
    fileInputRef.current?.click();
  };

  const handleWasmUploadClick = () => {
    wasmRef.current?.click();
  };

  return (
    <div className="flex items-center justify-center bg-[#033b41] rounded-[10pt] pl-8 pr-8 pt-10 pb-10 w-96 text-white">
      <form onSubmit={handleSubmit}>
        <div className="grid gap-2">
          <h1 className="text-[40px] font-bold pb-5">TACEO:Proof</h1>
          <div>
            <h2 className="text-[14pt] font-bold pb-1">Access Code</h2>
            <input required className="bg-white text-black rounded-[5pt] p-2 w-full" type="text" onChange={(e) => setCode(e.target.value)} />
          </div>
          <div>
            <h2 className="text-[14pt] font-bold pb-1">Blueprint</h2>
            <input required className="bg-white text-black rounded-[5pt] p-2 w-full" type="text" onChange={(e) => setBlueprint(e.target.value)} />
          </div>
          <div>
            <h2 className="text-[14pt] font-bold pb-1">Curve</h2>
            <select required className="bg-white text-black font-bold rounded-[5pt] p-2 w-full" onChange={(e) => setCurve(e.target.value as BlueprintCurve)}>
              <option value='Bn254'>Bn254</option>
              <option value='Bls381'>Bls12_381</option>
              <option value='Bls377'>Bls12_377</option>
            </select>
          </div>
          <div>
            <h2 className="text-[14pt] font-bold pb-1">Input</h2>
            <input
              type="file"
              ref={fileInputRef}
              onChange={handleInputFileChange}
              style={{ display: 'none' }}
            />
            <button className="bg-white text-black font-bold upload-trigger rounded-[5pt] p-2 w-full" onClick={handleInputUploadClick} type="button">
              {selectedFile ? selectedFile.name : 'Choose File'}
            </button>
          </div>
          <div>
            <h2 className="text-[14pt] font-bold pb-1">Job Type</h2>
            <select required className="bg-white text-black font-bold rounded-[5pt] p-2 w-full" onChange={(e) => setJobType(e.target.value as JobType)}>
              <option value='ShamirPove'>Shamir Prove</option>
              <option value='Rep3Pove'>REP3 Prove</option>
              <option value='Rep3Full'>Witness Extension + Prove</option>
            </select>
          </div>
          {jobType != JobType.Rep3Full && (
            <div>
              <h2 className="text-[14pt] font-bold pb-1">Circom WASM</h2>
              <input
                type="file"
                ref={wasmRef}
                onChange={handleWasmFileChange}
                style={{ display: 'none' }}
              />
              <button className="bg-white text-black font-bold upload-trigger rounded-[5pt] p-2 w-full" onClick={handleWasmUploadClick} type="button">
                {wasm ? wasm.name : 'Choose File'}
              </button>
            </div>
          )}
          {jobType == JobType.Rep3Full ?
            <div>
              <h2 className="text-[14pt] font-bold pb-1">Public Inputs</h2>
              <input required className="bg-white text-black rounded-[5pt] p-2 w-full" type="text" onChange={(e) => setPublicInputs(e.target.value.split(','))} />
            </div>
            :
            <div>
              <h2 className="text-[14pt] font-bold pb-1">Number of Inputs</h2>
              <input required className="bg-white text-black rounded-[5pt] p-2 w-full" type="text" onChange={(e) => setNumInputs(parseInt(e.target.value, 10))} />
            </div>
          }
          <div className="pt-5">
            <button className="bg-white text-black font-bold rounded-[5pt] p-2 w-full" type="submit" disabled={loading}>
               {loading ? ('Loading...') : 'Submit'}
            </button>
          </div>
          <div className="pt-5">
          {error && <div className="text-[#ff0000]">{error}</div>}
          {result && (
            <div>
              <a className="underline text-white" href={`data:text/json;charset=utf-8,${encodeURIComponent(JSON.stringify(result))}`} download="result.json">
                Download Result
              </a>
            </div>
          )}
          </div>
        </div>
      </form>
    </div>
  );
}
