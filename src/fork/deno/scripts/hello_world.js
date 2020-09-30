let bootstrap = this.__bootstrap;
function main() {
    Deno.core.print("Hello, World!");
    Deno.core.ops();

    Deno.core.print(client_rid);
    Deno.core.jsonOpAsync("fetch", {client_rid, url: 'https://raw.githubusercontent.com/Kethku/neovide/master/LICENSE'}).then(resp => {Deno.core.print(resp); Deno.core.print(asd)} ).catch(e => Deno.core.print(e.stack));
    Deno.core.print("ioashdoiasdoiahsdoiahsdoi");
}

main();