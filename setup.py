from setuptools import setup

from setuptools_rust import Binding, RustExtension

setup(
    name="hephaestus",
    version="0.1.0",
    rust_extensions=[RustExtension("hephaestus.hephaestus", binding=Binding.PyO3)],
    packages=["hephaestus"],
    # rust extensions are not zip safe, just like C-extensions.
    zip_safe=False,
)
