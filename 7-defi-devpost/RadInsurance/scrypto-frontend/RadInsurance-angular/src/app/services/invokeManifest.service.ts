import { Injectable } from '@angular/core';
import { Rdt } from 'src/app/types';
import { ConnectionService } from '../layout/components/connection-button/connection.service';

@Injectable({
  providedIn: 'root',
})
export class InvoqueManifestService {

private rdt!: Rdt;
  constructor(
    private connectionService: ConnectionService,
  ) {
    setTimeout(() => {
        this.rdt = this.connectionService.getRdt();
      }, 500);
  }
  public  async invokeManifest(manifest: string) : Promise<string> {
    console.log('init component manifest : ', manifest);

    const result = await this.rdt.sendTransaction({
      transactionManifest: manifest,
      version: 1,
    });

    if (result.isErr()) {
      console.log('transaction id : ', result.error);
      throw result.error;
    }

    console.log('Success : ', result);
    return result.value.transactionIntentHash; 

  }
 
}
