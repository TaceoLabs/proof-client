import { Navigate, NavLink, Route, Routes, type NavLinkRenderProps } from "react-router-dom";
import CoCircomFull from "./pages/CoCircomFull";
import CoCircomProve from "./pages/CoCircomProve";
import CoNoirFull from "./pages/CoNoirFull";
import CoNoirProve from "./pages/CoNoirProve";
import CoCircomFullMultipleInputs from "./pages/CoCircomFullMultiInput";
import CoCircomAddJobInputs from "./pages/CoCircomAddJobInputs";
import CoCircomGetJobResults from "./pages/CoCircomGetJobResult";

const navLinkStyle = (navData: NavLinkRenderProps) => (navData.isActive ? 'block w-full text-left p-4 rounded-[10pt] bg-[#52ffc5]' : 'block w-full text-left p-4 rounded-[10pt] hover:bg-[#52ffc5]');

export default function Home() {
  return (
    <div className="h-screen">
      <nav className="fixed top-0 left-0 h-full max-w-fit flex flex-col items-center border-r shadow-xl text-[20px] font-bold p-4 space-y-1">
        <NavLink className={navLinkStyle} to="/co-circom-prove">CoCircom Prove</NavLink>
        <NavLink className={navLinkStyle} to="/co-circom-full">CoCircom Full</NavLink>
        <NavLink className={navLinkStyle} to="/co-circom-full-multiple-inputs">CoCircom Full Multiple Inputs</NavLink>
        <NavLink className={navLinkStyle} to="/co-circom-add-job-inputs">CoCircom Add Inputs</NavLink>
        <NavLink className={navLinkStyle} to="/co-circom-get-job-result">CoCircom Get Job Result</NavLink>
        <NavLink className={navLinkStyle} to="/co-noir-prove">CoNoir Prove</NavLink>
        <NavLink className={navLinkStyle} to="/co-noir-full">CoNoir Full</NavLink>
      </nav>
      <main className="flex items-center justify-center h-screen">
        <div className="flex items-center justify-center text-xl">
          <Routes>
            <Route path="/" element={<Navigate to="/co-circom-prove" replace />} />
            <Route path="/co-circom-prove" element={<CoCircomProve />} />
            <Route path="/co-circom-full" element={<CoCircomFull />} />
            <Route path="/co-circom-full-multiple-inputs" element={<CoCircomFullMultipleInputs />} />
            <Route path="/co-circom-add-job-inputs" element={<CoCircomAddJobInputs />} />
            <Route path="/co-circom-get-job-result" element={<CoCircomGetJobResults />} />
            <Route path="/co-noir-prove" element={<CoNoirProve />} />
            <Route path="/co-noir-full" element={<CoNoirFull />} />
          </Routes>
        </div>
      </main>
    </div>
  );
}
