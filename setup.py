from setuptools import setup, find_packages
import pathlib

here = pathlib.Path(__file__).parent.resolve()
long_description = (here / 'README.md').read_text(encoding='utf-8')

setup(
    name='folkol-utils',
    version='0.0.4',
    description='meta-package for various command line utilities (folkol.xxx)',
    long_description=long_description,
    long_description_content_type='text/markdown',
    url='https://github.com/folkol/futils',
    author='folkol',
    classifiers=[
        'Development Status :: 3 - Alpha',
        'Intended Audience :: Developers',
        'Topic :: Terminals',
        'Topic :: Text Processing :: Filters',
        'License :: OSI Approved :: MIT License',
    ],
    install_requires=[
        'folkol.grab',
        'folkol.yg',
    ],
    keywords='shell, unix filter',
    python_requires='>=3.6, <4',
    project_urls={
        'Bug Reports': 'https://github.com/folkol/futils/issues',
        'Source': 'https://github.com/folkol/futils',
    },
)
