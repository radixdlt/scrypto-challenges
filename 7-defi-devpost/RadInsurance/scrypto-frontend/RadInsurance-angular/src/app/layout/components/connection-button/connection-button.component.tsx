import { Component } from "@angular/core";
import * as ReactDOM from "react-dom/client";
import * as React from "react";
import { ConnectionService } from "./connection.service";
@Component({
  selector: "app-connection-button",
  templateUrl: "./connection-button.component.html",
  styleUrls: ["./connection-button.component.css"],
})
export class ConnectionButtonComponent {
  private rdt = this.connectionService.getRdt();
  ngAfterContentInit() {
    this.render();
  }
  private render() {
    const doc: HTMLElement | null = document.getElementById(
      "radix-button"
    ) as HTMLElement;
    const root = ReactDOM.createRoot(doc);
    root.render(<radix-connect-button></radix-connect-button>);
  }

  constructor(private connectionService: ConnectionService) {}
}

declare global {
  namespace JSX {
    interface IntrinsicElements {
      "radix-connect-button": React.DetailedHTMLProps<
        React.HTMLAttributes<HTMLElement>,
        HTMLElement
      >;
    }
  }
}
