# loose-semver

> [!WARNING]  
> This package does not provide a spec-accurate implementation of Semantic Versioning. If you are 
> looking for something like that, please take a look at the [semver](https://crates.io/crates/semver) 
> or [semver-rs](https://crates.io/crates/semver-rs) packages.

This crate allows to parse "semantic versions" that are not formatted exactly like spec. For example, the version scheme used by Go or Minecraft is an example for this.

This crate allows to parse versions following the [Sematic Version](https://semver.org) spec as well
as formats like the following examples:

- `v1.2`
- `3.4.5beta1`
- `2.3-alpha.3`
- `5-rc.1.2.3`

## Example

```rs
use loose_semver::Version;

fn main() {
    let v: Version = "1.2.3rc4".parse().unwrap();
    dbg!(v);
}
```