# flashyfi-frontend

The demo frontend is publicly available at https://backyard-coder.github.io/flashyfi

If the public website has fallen victim to the ruthless grasp of the internet, or you just feel like running a test
locally, simply execute the bellow commands.

```shell
npm install
npm run dev -- --open
```

A possible failure mode is users smuggling data onto the Radix network that this frontend is not equipped to handle. I
have taken some measures to guard against adversarial behavior but if those prove ineffective, you will probably have to
perform your tests on a newly instantiated component of the FlashyFi blueprint.

To instantiate a new component, submit the following manifest:

```
CALL_FUNCTION
    PackageAddress("package_tdx_b_1q9lyy7eyqj05z8sywc82ajaxfzsrmcn3a4h9mmc063nsp0nvjg")
    "Flashyfi"
    "instantiate_global"
    false;
```

You will then have to update the value of the `FLASHYFI_COMPONENT_ADDRESS` variable under `src/lib/constants` with the
new component address.
