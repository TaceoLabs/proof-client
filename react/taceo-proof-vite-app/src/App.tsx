import React, { type ChangeEvent, useRef, useState } from "react";
import { scheduleCoCircomFullJobRep3, scheduleCoCircomProveJobShamir, scheduleCoCircomProveJobRep3, fetchJobResult, type ConfigurationParameters, JobApi, NodeApi, Configuration, JobType, BlueprintCurve, scheduleCoNoirFullJobRep3, scheduleCoNoirProveJobShamir, scheduleCoNoirProveJobRep3 } from '@taceo/proof-client-bundler'
import wc from "./witness_calculator.js"; // generated with circom

// const apiUrl = "https://proof.taceo.network";
// const wsUrl = "wss://proof.taceo.network/api/v1/reports/subs";
const apiUrl = "http://localhost:1234";
const wsUrl = "ws://localhost:1234/api/v1/reports/subs";

type CoSnark = "CoCicom" | "CoNoir";
type WitnessExtension = "Upload" | "Browser";

const configParams: ConfigurationParameters = {
  basePath: apiUrl,
}
const configuration = new Configuration(configParams)
const jobInstance = new JobApi(configuration);
const nodeInstance = new NodeApi(configuration);

export default function Home() {
  const [voucher, setVoucher] = useState<string | null>(null);
  const [blueprint, setBlueprint] = useState<string>("");
  const [curve, setCurve] = useState<BlueprintCurve>(BlueprintCurve.Bn254);
  const [proof, setProof] = useState<string | null>(null);
  const [publicInputsOut, setPublicInputsOut] = useState<string | null>(null);
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
  const [coSnark, setCoSnark] = useState<CoSnark>("CoCicom");
  const [abi, setAbi] = useState<File | null>(null);
  const abiRef = useRef<HTMLInputElement>(null);
  const [publicInputIndices, setPublicInputIndices] = useState<Array<number>>([]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    setError(null);
    setProof(null);
    setPublicInputsOut(null);

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
      const nodes = await nodeInstance.randomNodeProviders();
      if (coSnark == "CoCicom") {
        if (jobType == JobType.Rep3Full) {
          input = JSON.parse(await selectedFile!.text());
          jobId = await scheduleCoCircomFullJobRep3(jobInstance, nodes, blueprint, voucher, curve, publicInputs, input);
        } else {
          if (wtnsExt == "Browser") {
            input = JSON.parse(await selectedFile!.text());
            witnessCalculator = await wc(new Uint8Array(await wasm!.arrayBuffer()));
            witness = await witnessCalculator.calculateWTNSBin(input, 0);
          } else {
            witness = new Uint8Array(await selectedFile!.arrayBuffer());
          }
          if (jobType == JobType.ShamirProve) {
            jobId = await scheduleCoCircomProveJobShamir(jobInstance, nodes, blueprint, voucher, curve, numInputs, witness);
          } else {
            jobId = await scheduleCoCircomProveJobRep3(jobInstance, nodes, blueprint, voucher, curve, numInputs, witness);
          }
        }
      } else {
        const publicInputIndicesArray = new Uint32Array(publicInputIndices);
        if (jobType == JobType.Rep3Full) {
          input = JSON.parse(await selectedFile!.text());
          const parsedAbi = JSON.parse(await abi!.text());
          jobId = await scheduleCoNoirFullJobRep3(jobInstance, nodes, blueprint, voucher, parsedAbi, publicInputIndicesArray, input);
        } else {
          witness = new Uint8Array(await selectedFile!.arrayBuffer());
          if (jobType == JobType.ShamirProve) {
            jobId = await scheduleCoNoirProveJobShamir(jobInstance, nodes, blueprint, voucher, publicInputIndicesArray, witness);
          } else {
            jobId = await scheduleCoNoirProveJobRep3(jobInstance, nodes, blueprint, voucher, publicInputIndicesArray, witness);
          }
        }
      }
      const jobResult = await fetchJobResult(wsUrl, jobId);
      setProof(jobResult.proof);
      setPublicInputsOut(jobResult.public_inputs);
      setLoading(false);
    } catch (error: any) {
      setError(error.message);
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

  const handleAbiFileChange = (e: ChangeEvent<HTMLInputElement>) => {
    const files = e.target.files;
    if (files && files.length > 0) {
      setAbi(files[0]);
    }
  };

  const handleInputUploadClick = () => {
    fileInputRef.current?.click();
  };

  const handleWasmUploadClick = () => {
    wasmRef.current?.click();
  };

  const handleAbiUploadClick = () => {
    abiRef.current?.click();
  };

  return (
    <div className="flex items-center justify-center rounded-[10pt] shadow-xl border border-current py-10 px-15 w-lg">
      <form className="w-full" onSubmit={handleSubmit}>
        <div className="grid gap-2">
          <h1 className="text-[40px] font-bold text-center">TACEO:Proof</h1>
          <div className="w-[5rem] h-[1rem] bg-[#52ffc5] mx-auto my-5"></div>
          <div>
            <h2 className="text-[14pt] font-bold pb-1">CoSNARK</h2>
            <select required className="rounded-[5pt] shadow-xl border border-current p-2 w-full" onChange={(e) => setCoSnark(e.target.value as CoSnark)}>
              <option value='CoCicom'>CoCicom</option>
              <option value='CoNoir'>CoNoir</option>
            </select>
          </div>
          <div>
            <h2 className="text-[14pt] font-bold pb-1">Voucher</h2>
            <input className="rounded-[5pt] shadow-xl border border-current p-2 w-full" type="text" onChange={(e) => setVoucher(e.target.value)} />
          </div>
          <div>
            <h2 className="text-[14pt] font-bold pb-1">Blueprint</h2>
            <input required className="rounded-[5pt] shadow-xl border border-current p-2 w-full" type="text" onChange={(e) => setBlueprint(e.target.value)} />
          </div>
          {coSnark == "CoCicom" && (
            <div>
              <h2 className="text-[14pt] font-bold pb-1">Curve</h2>
              <select required className="rounded-[5pt] shadow-xl border border-current p-2 w-full" onChange={(e) => setCurve(e.target.value as BlueprintCurve)}>
                <option value='Bn254'>BN254</option>
                <option value='Bls381'>BLS12_381</option>
                <option value='Bls377'>BLS12_377</option>
              </select>
            </div>
          )}
          <div>
            <h2 className="text-[14pt] font-bold pb-1">Job Type</h2>
            <select required className="rounded-[5pt] shadow-xl border border-current p-2 w-full" onChange={(e) => setJobType(e.target.value as JobType)}>
              <option value='ShamirProve'>Shamir Prove</option>
              <option value='Rep3Prove'>REP3 Prove</option>
              <option value='Rep3Full'>Witness Extension + Prove</option>
            </select>
          </div>
          {coSnark == "CoCicom" && jobType != JobType.Rep3Full && (
            <div>
              <h2 className="text-[14pt] font-bold pb-1">Witness Extension</h2>
              <select required className="rounded-[5pt] shadow-xl border border-current p-2 w-full" onChange={(e) => setWtnsExt(e.target.value as WitnessExtension)}>
                <option value='Upload'>Upload Witness</option>
                <option value='Browser'>Compute in Browser</option>
              </select>
            </div>
          )}
          {coSnark == "CoCicom" && jobType != JobType.Rep3Full && wtnsExt == "Browser" && (
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
          {coSnark == "CoCicom" && jobType == JobType.Rep3Full && (
            <div>
              <h2 className="text-[14pt] font-bold pb-1">Public Inputs</h2>
              <input className="rounded-[5pt] shadow-xl border border-current p-2 w-full" type="text" onChange={(e) => setPublicInputs(e.target.value.split(','))} />
            </div>
          )}
          {coSnark == "CoCicom" && jobType != JobType.Rep3Full && (
            <div>
              <h2 className="text-[14pt] font-bold pb-1">Number of Inputs</h2>
              <input required className="rounded-[5pt] shadow-xl border border-current p-2 w-full" type="number" onChange={(e) => setNumInputs(parseInt(e.target.value, 10))} />
            </div>
          )}
          {coSnark == "CoNoir" && jobType == JobType.Rep3Full && (
            <div>
              <h2 className="text-[14pt] font-bold pb-1">Abi</h2>
              <input
                type="file"
                ref={abiRef}
                onChange={handleAbiFileChange}
                style={{ display: 'none' }}
              />
              <button className="rounded-[5pt] shadow-xl border border-current p-2 w-full cursor-pointer" onClick={handleAbiUploadClick} type="button">
                {abi ? abi.name : 'Choose File'}
              </button>
            </div>
          )}
          {coSnark == "CoNoir" && (
            <div>
              <h2 className="text-[14pt] font-bold pb-1">Public Input Indices</h2>
              <input className="rounded-[5pt] shadow-xl border border-current p-2 w-full" type="text" onChange={(e) => setPublicInputIndices(e.target.value.split(',').map((e) => parseInt(e)))} />
            </div>
          )}
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
                {coSnark == "CoCicom" && (
                  <div>
                    <a className="underline text-current" href={`data:text/json;charset=utf-8,${encodeURIComponent(proof!)}`} download="proof.json">
                      Download Proof
                    </a>
                    <br />
                    <a className="underline text-current" href={`data:text/json;charset=utf-8,${encodeURIComponent(publicInputsOut!)}`} download="public.json">
                      Download Public Inputs
                    </a>
                  </div>
                )}
                {coSnark == "CoNoir" && (
                  <div>
                    <a className="underline text-current" href={`data:application/octet-stream;base64,${proof!}`} download="proof">
                      Download Proof
                    </a>
                    <br />
                    <a className="underline text-current" href={`data:application/octet-stream;base64,${publicInputsOut!}`} download="public_inputs">
                      Download Public Inputs
                    </a>
                  </div>
                )}
              </div>
            )}
          </div>
        </div>
      </form>
    </div>
  );
}
