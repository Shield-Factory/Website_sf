import init from "/home/caluri0/Documents/website_sf/pkg/website_sf.js";

const runWasm = async () => {
  const shieldFactory = await init("pkg/website_sf_bg.wasm");

  const addResult = shieldFactory.add(24,24);

  document.getElementById("result")
    .textContent='Le résultat est ' + addResult;
};
runWasm();