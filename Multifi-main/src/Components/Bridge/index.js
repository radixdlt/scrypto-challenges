import React, { useEffect } from 'react';
import { Widget } from '@maticnetwork/wallet-widget';

const widget = new Widget({
  target: '#polyBridge',
  appName: 'Bridge',
  autoShowTime: 0,
  position: 'center',
  height: 618,
  width: 550,
  overlay: false,
  network: 'testnet',
  closable: true,
});
widget.create();

function Bridge() {

  useEffect(() => {
    widget.show();
  }, [])

  useEffect(() => {
    return () => {
      if(widget.isVisible) widget.hide();
    }
  }, [])
  
  return(
    <div></div>
  )
}

export default Bridge;