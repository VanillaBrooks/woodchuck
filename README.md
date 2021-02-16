# woodchuck

Convert CSV files into latex tables

## Usage

`woodchuck --help`

```
Woodchuck 0.1.0
Brooks
Convert CSV files to latex tables

USAGE:
    woodchuck [ARGS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <FILE>    Input path to csv
    <FILE>    Path to .tex file to write to

```


## Example

Input `table.csv`:

```
No.,"Flow meter reading, Q [$\frac{m^3}{s}$]",$h_1$ [m],$h_2$ [m],$\sqrt[]{h_1 - h_2}$ $\sqrt[]{m}$,C
1,0.000375,0.161,0.013,0.385,1.01
2,0.0003407,0.159,0.039,0.346,1.02
3,0.0003147,0.156,0.056,0.316,1.03
4,0.0002898,0.156,0.072,0.29,1.04
5,0.0002557,0.158,0.091,0.259,1.03
6,0.0001807,0.156,0.123,0.182,1.03
```

Running:

`woodchuck table.csv table.tex`

Generated `table.tex`:

```
\begin{table}[H]
	\begin{center}
		\begin{tabular} { |c|c|c|c|c|c| }
			\hline
			No. & Flow meter reading, Q [$\frac{m^3}{s}$] & $h_1$ [m] & $h_2$ [m] & $\sqrt[]{h_1 - h_2}$ $[m^{1/2}]$ & C\\ \hline
			1 & 0.0003750 & 0.161 & 0.013 & 0.385 & 1.01\\ \hline
			2 & 0.0003407 & 0.159 & 0.039 & 0.346 & 1.02\\ \hline
			3 & 0.0003147 & 0.156 & 0.056 & 0.316 & 1.03\\ \hline
			4 & 0.0002898 & 0.156 & 0.072 & 0.29 & 1.04\\ \hline
			5 & 0.0002557 & 0.158 & 0.091 & 0.259 & 1.03\\ \hline
			6 & 0.0001807 & 0.156 & 0.123 & 0.182 & 1.03\\ \hline
		\end{tabular}
		\caption{Caption here}
		\label{labelhere}
	\end{center}
\end{table}

```
