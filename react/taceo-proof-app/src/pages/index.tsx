import React, { ChangeEvent, useRef, useState } from "react";
import { BlueprintApi, BlueprintCurve, Configuration, ConfigurationParameters, JobApi, JobStatus, JobType, NpsKeyMaterial, ProofResult } from '@taceo/proof-api-client';
import { scheduleFullJobRep3, scheduleProveJobShamir, scheduleProveJobRep3, verifyProofResultSignature} from '@taceo/proof-client-browser'
import wc from "../witness-calculator.js"; // generated with circom

type WitnessExtension = "Upload" | "Browser";

const configParams: ConfigurationParameters = {
  basePath: "http://localhost:1234",
}
const congiuration = new Configuration(configParams)
const jobInstance = new JobApi(congiuration);
const blueprintInstance = new BlueprintApi(congiuration);

export default function Home() {
  const [code, setCode] = useState<string | null>(null);
  const [blueprint, setBlueprint] = useState<string>("");
  const [curve, setCurve] = useState<BlueprintCurve>(BlueprintCurve.Bn254);
  const [result, setResult] = useState<ProofResult | null>(null);
  const [loading, setLoading] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);
  const [jobType, setJobType] = useState<JobType>(JobType.ShamirProve);
  const [numInputs, setNumInputs] = useState<number>(0);
  const [publicInputs, setPublicInputs] = useState<Array<string>>([]);
  const [selectedFile, setSelectedFile] = useState<File | null>(null);
  const fileInputRef = useRef<HTMLInputElement>(null);
  const [wasm, setWasm] = useState<File | null>(null);
  const wasmRef = useRef<HTMLInputElement>(null);
  const [wtnsExt, setWtnsExt] = useState<WitnessExtension>("Upload");

  const pollProofResult = async (jobId: string, keyMaterial: NpsKeyMaterial[]): Promise<ProofResult | null> => {
    while (true) {
      try {
        const jobResult = await jobInstance.getResult({ id: jobId });
        if (jobResult.status == JobStatus.Success && jobResult.signature0 != null && jobResult.signature1 != null && jobResult.signature2 != null) {
          const proofResult = jobResult.ok!;
          verifyProofResultSignature(jobId, proofResult, jobResult.signature0, keyMaterial[0]);
          verifyProofResultSignature(jobId, proofResult, jobResult.signature1, keyMaterial[1]);
          verifyProofResultSignature(jobId, proofResult, jobResult.signature2, keyMaterial[2]);
          return proofResult;
        } else if (jobResult.status == JobStatus.Failed) {
          setError(jobResult.error ?? "something went wrong");
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

    if (wasm == null && jobType != JobType.Rep3Full && wtnsExt == "Browser") {
      setError("wasm file missing");
      return;
    }

    let input;
    let jobId;
    let witnessCalculator;
    let witness;

    setLoading(true);

    try {
      const keyMaterial = await blueprintInstance.blueprintKeyMaterial({id: blueprint});
      if (jobType == JobType.Rep3Full) {
        input = JSON.parse(await selectedFile!.text());
        jobId = await scheduleFullJobRep3(jobInstance, blueprint, code, curve, keyMaterial, publicInputs, input);
      } else {
        if (wtnsExt == "Browser") {
          input = JSON.parse(await selectedFile!.text());
          witnessCalculator = await wc(await wasm!.bytes());
          witness = await witnessCalculator.calculateWTNSBin(input, 0);
        } else {
          witness = await selectedFile!.bytes();
        }
        if (jobType == JobType.ShamirProve) {
          jobId = await scheduleProveJobShamir(jobInstance, blueprint, code, curve, keyMaterial, numInputs, witness);
        } else {
          jobId = await scheduleProveJobRep3(jobInstance, blueprint, code, curve, keyMaterial, numInputs, witness);
        }
      }
      const result = await pollProofResult(jobId, keyMaterial);
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
    <div className="flex items-center justify-center rounded-[10pt] shadow-xl border border-current py-10 px-15 w-lg">
      <form className="w-full" onSubmit={handleSubmit}>
        <div className="grid gap-2">
          <h1 className="text-[40px] font-bold text-center">TACEO:Proof</h1>
          <div className="w-[5rem] h-[1rem] bg-[#52ffc5] mx-auto my-5"></div>
          <div>
            <h2 className="text-[14pt] font-bold pb-1">Access Code</h2>
            <input className="rounded-[5pt] shadow-xl border border-current p-2 w-full" type="text" onChange={(e) => setCode(e.target.value)} />
          </div>
          <div>
            <h2 className="text-[14pt] font-bold pb-1">Blueprint</h2>
            <input required className="rounded-[5pt] shadow-xl border border-current p-2 w-full" type="text" onChange={(e) => setBlueprint(e.target.value)} />
          </div>
          <div>
            <h2 className="text-[14pt] font-bold pb-1">Curve</h2>
            <select required className="rounded-[5pt] shadow-xl border border-current p-2 w-full" onChange={(e) => setCurve(e.target.value as BlueprintCurve)}>
              <option value='Bn254'>BN254</option>
              <option value='Bls381'>BLS12_381</option>
              <option value='Bls377'>BLS12_377</option>
            </select>
          </div>
          <div>
            <h2 className="text-[14pt] font-bold pb-1">Job Type</h2>
            <select required className="rounded-[5pt] shadow-xl border border-current p-2 w-full" onChange={(e) => setJobType(e.target.value as JobType)}>
              <option value='ShamirProve'>Shamir Prove</option>
              <option value='Rep3Prove'>REP3 Prove</option>
              <option value='Rep3Full'>Witness Extension + Prove</option>
            </select>
          </div>
          {jobType != JobType.Rep3Full && (
            <div>
              <h2 className="text-[14pt] font-bold pb-1">Witness Extension</h2>
              <select required className="rounded-[5pt] shadow-xl border border-current p-2 w-full" onChange={(e) => setWtnsExt(e.target.value as WitnessExtension)}>
                <option value='Upload'>Upload Witness</option>
                <option value='Browser'>Compute in Browser</option>
              </select>
            </div>
          )}
          {jobType != JobType.Rep3Full && wtnsExt == "Browser" && (
            <div>
              <h2 className="text-[14pt] font-bold pb-1">Circom WASM</h2>
              <input
                type="file"
                ref={wasmRef}
                onChange={handleWasmFileChange}
                style={{ display: 'none' }}
              />
              <button className="rounded-[5pt] shadow-xl border border-current p-2 w-full cursor-pointer" onClick={handleWasmUploadClick} type="button">
                {wasm ? wasm.name : 'Choose File'}
              </button>
            </div>
          )}
          <div>
            <h2 className="text-[14pt] font-bold pb-1">
              {jobType == JobType.Rep3Full || wtnsExt == "Browser" ? 'Input' : 'Witness'}
            </h2>
            <input
              type="file"
              ref={fileInputRef}
              onChange={handleInputFileChange}
              style={{ display: 'none' }}
            />
            <button className="rounded-[5pt] shadow-xl border border-current p-2 w-full cursor-pointer" onClick={handleInputUploadClick} type="button">
              {selectedFile ? selectedFile.name : 'Choose File'}
            </button>
          </div>
          {jobType == JobType.Rep3Full ?
            <div>
              <h2 className="text-[14pt] font-bold pb-1">Public Inputs</h2>
              <input required className="rounded-[5pt] shadow-xl border border-current p-2 w-full" type="text" onChange={(e) => setPublicInputs(e.target.value.split(','))} />
            </div>
            :
            <div>
              <h2 className="text-[14pt] font-bold pb-1">Number of Inputs</h2>
              <input required className="rounded-[5pt] shadow-xl border border-current p-2 w-full" type="number" onChange={(e) => setNumInputs(parseInt(e.target.value, 10))} />
            </div>
          }
          <div className="pt-8 mx-auto">
            {loading ?
              <button className="text-[14pt] text-black font-bold rounded-[5pt] bg-[#52ffc5] py-2 pl-3 pr-5 inline-flex items-center" type="submit" disabled={true}>
                <svg className="mr-3 ml-1 size-6 animate-spin" viewBox="0 0 64 64">
                  <circle fill="none" strokeWidth="10" className="stroke-black opacity-40" cx="32" cy="32" r="24" />
                  <circle fill="none" strokeWidth="10" className="stroke-black" strokeDasharray="250" strokeDashoffset="210" cx="32" cy="32" r="24"/>
                </svg>
                Loading...
              </button>
              :
              <button className="text-[14pt] text-black font-bold rounded-[5pt] bg-[#52ffc5] p-2 cursor-pointer px-12" type="submit">
                Submit
              </button>
            }
          </div>
          <div className="pt-5 mx-auto">
            {error && <div className="text-[#ff0000]">{error}</div>}
            {result && (
              <div>
                <a className="underline text-current" href={`data:text/json;charset=utf-8,${encodeURIComponent(JSON.stringify(result))}`} download="result.json">
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
