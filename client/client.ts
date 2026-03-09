import * as anchor from "@coral-xyz/anchor";
import { PublicKey, SystemProgram } from "@solana/web3.js";

// ID del programa (owner declarado en tu código Rust)
const PROGRAM_ID = new PublicKey("OWNER111111111111111111111111111111111111111");

// Configuración del proveedor (wallet + conexión)
const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

// Cargar el IDL generado por Anchor
import idl from "./target/idl/veterinaria.json";

// Crear la instancia del programa
const program = new anchor.Program(idl as anchor.Idl, PROGRAM_ID, provider);

// Función para crear una mascota
export async function crearMascota(
  nombre: string,
  raza: string,
  especie: string,
  edad: number,
  vacunacion: string,
  dueno: string,
  enfermedades: string
) {
  const [cuentaMascota] = PublicKey.findProgramAddressSync(
    [Buffer.from("mascota"), provider.wallet.publicKey.toBuffer(), Buffer.from(nombre)],
    program.programId
  );

  await program.methods
    .crearMascota(nombre, raza, especie, edad, vacunacion, dueno, enfermedades)
    .accounts({
      cuentaMascota,
      usuario: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
    })
    .rpc();

  console.log(`Mascota ${nombre} creada en la cuenta: ${cuentaMascota.toBase58()}`);
}

// Función para actualizar una mascota
export async function actualizarMascota(
  nombre: string,
  raza: string,
  especie: string,
  edad: number,
  vacunacion: string,
  dueno: string,
  enfermedades: string
) {
  const [cuentaMascota] = PublicKey.findProgramAddressSync(
    [Buffer.from("mascota"), provider.wallet.publicKey.toBuffer(), Buffer.from(nombre)],
    program.programId
  );

  await program.methods
    .actualizarMascota(nombre, raza, especie, edad, vacunacion, dueno, enfermedades)
    .accounts({
      cuentaMascota,
      usuario: provider.wallet.publicKey,
    })
    .rpc();

  console.log(`Mascota ${nombre} actualizada.`);
}

// Función para obtener datos de una mascota
export async function obtenerMascota(nombre: string) {
  const [cuentaMascota] = PublicKey.findProgramAddressSync(
    [Buffer.from("mascota"), provider.wallet.publicKey.toBuffer(), Buffer.from(nombre)],
    program.programId
  );

  const cuenta = await program.account.cuentaMascota.fetch(cuentaMascota);
  console.log("Datos de la mascota:", cuenta);
}

// Función para eliminar una mascota
export async function eliminarMascota(nombre: string) {
  const [cuentaMascota] = PublicKey.findProgramAddressSync(
    [Buffer.from("mascota"), provider.wallet.publicKey.toBuffer(), Buffer.from(nombre)],
    program.programId
  );

  await program.methods
    .eliminarMascota()
    .accounts({
      cuentaMascota,
      usuario: provider.wallet.publicKey,
    })
    .rpc();

  console.log(`Mascota ${nombre} eliminada.`);
}
