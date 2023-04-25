import * as wasm from "../../../pkg/rust_bg.wasm";

export default function Home() {
  console.log(wasm.doTheThing());
  return (
    <>
      <div>test</div>
    </>
  );
}
