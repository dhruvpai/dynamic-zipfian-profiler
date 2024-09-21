# dynamic-zipfian-profiler

This program takes a dataset in CSV format and uses the Levenberg-Marquardt algorithm to determine how Zipfian the distribution is following the analysis presented by Giordano De Marzo, Andrea Gabrielli, Andrea Zaccaria, and Luciano Pietronero. This kind of analysis can be useful across a wide range of systems where Zipfian distributions naturally exist, with applications in language, socio-economic systems, statistics, forecasting, and understanding low-probability/high-impact (black swan) events.

## Background

Zipfian distributions behave according to the Zipf-Mandelbrot relation $S(k) = \frac{\overline{S}}{(k + Q)^{\gamma}}$, where $S(k)$ is an approximation of the size of the $k$ th largest object in a dataset from a Zipfian distribution, and tend to follow the probability density function defined by $P(S) = 0$ for $S < s_{m}$, $P(S) = \frac{c}{S^{\alpha}}$ for $s_{m} \leq S \leq s_{M}$, and $P(S) = 0$ for $S > s_{M}$. As discussed in the paper ["Dynamical approach to Zipf’s law"](https://arxiv.org/pdf/1911.04844) by Giordano De Marzo, Andrea Gabrielli, Andrea Zaccaria, and Luciano Pietronero, we can derive the equalities $\gamma = \frac{1}{\alpha - 1}$, $\overline{S} = N^{\gamma}s_{m}$, and $Q = N(\frac{s_{m}}{s_{M}})^{\frac{1}{\gamma}}$, which we can then substitute into the original Zipf-Mandelbrot relation to obtain the modified Zipf-Mandelbrot relation $S(k) = \frac{N^{\gamma}s_{m}}{(k + N(\frac{s_{m}}{s_{M}})^{\alpha - 1})^{\frac{1}{\alpha - 1}}}$, where $N$ is the number of objects in the dataset. This allows us to obtain the equation $\log{S(k)} = -\frac{1}{\alpha - 1}\log(k + N(\frac{s_{m}}{s_{M}})^{\alpha - 1}) + \log(N^{\frac{1}{\alpha - 1}}s_{m})$, which we can then use to fit a curve based on the Zipf-Mandelbrot relation to a given dataset via a version of the [Levenberg-Marquardt algorithm](https://en.wikipedia.org/wiki/Levenberg%E2%80%93Marquardt_algorithm) with respect to the parameters $\alpha$, $s_{m}$, and $s_{M}$ that is regularized to prefer values of $s_{m}$ that are closer to the minimum size from the dataset and values of $s_{M}$ that are closer to the maximum size from the dataset. This code makes use of the previously described curve-fitting strategy in order to repeatedly test whether $\frac{dQ}{dn} \leq 0$, where $n = \sum_{k = 1}^{N} S(k)$. The code is expected to detect that $\frac{dQ}{dn} \leq 0$ more frequently for datasets that come from more Zipfian distributions and detect that $\frac{dQ}{dn} \leq 0$ less frequently for datasets that come from less Zipfian distributions.

## How to Use

This code operates on datasets in CSV format. To run the code, use the command "cargo run -- [name of CSV file] [index for column of data to be used] [hyperparameter for regularizing $s_{m}$] [hyperparameter for regularizing $s_{M}$]". For example, to use the populations of US cities in 2010 from the "county2010_hist_pops.csv" file as the data for the code without any regularization, we should use the following command:

```bash
cargo run -- county2010_hist_pops.csv 26 0.0 0.0
```

Running the code results in the code printing out information such as the equations for the Zipf-Mandelbrot relation and the probability density function of the Zipfian distribution that each of the cumulative samples of the dataset is assumed to come from, along with a count of how many times it was detected that $\frac{dQ}{dn} \leq 0$, as shown below with the results of running the code using the aforementioned example command:

```bash
SETUP TO READ COMPLETE
READ COMPLETE
Pre-Counted Float data found in CSV file.

i: 1; samples length: 157
made it out the get_samples match (GOOD)
Alpha: 1.240212140434481; Min: 24397.465427522853; Max: 414344.5851632488
Q: 79.51236048159616; n: 9881450.39003137
S(k) = 33794517809174.875/((k + 79.51236048159616)^4.162986925603598); P(S) = 0 for S < 24397.465427522853, P(S) = 5.510051246842354/(S^1.240212140434481) for 24397.465427522853 <= S <= 414344.5851632488, and P(S) = 0 for S > 414344.5851632488; S(1) + ... + S(157) = 9881450.39003137; v = 1.5129417130559404

i: 2; samples length: 314
made it out the get_samples match (GOOD)
Alpha: 1.2770338897012978; Min: 7928.0641815008075; Max: 1931018.0897881538
Q: 68.51020263180231; n: 49168969.984147355
dQ/dn <= 0
S(k) = 8170542667822.682/((k + 68.51020263180231)^3.609666676803388); P(S) = 0 for S < 7928.0641815008075, P(S) = 4.262167808772566/(S^1.2770338897012978) for 7928.0641815008075 <= S <= 1931018.0897881538, and P(S) = 0 for S > 1931018.0897881538; S(1) + ... + S(314) = 49168969.984147355; v = 7.208892847060539

i: 3; samples length: 471
made it out the get_samples match (GOOD)
Alpha: 1.243195351801304; Min: 13873.05960617815; Max: 1608266.4037434468
Q: 148.25881702604804; n: 74927216.1411691
dQ/dn > 0
S(k) = 1359634695348907.8/((k + 148.25881702604804)^4.1119206950016975); P(S) = 0 for S < 13873.05960617815, P(S) = 3.609764160233219/(S^1.243195351801304) for 13873.05960617815 <= S <= 1608266.4037434468, and P(S) = 0 for S > 1608266.4037434468; S(1) + ... + S(471) = 74927216.1411691; v = 11.469755605956143

i: 4; samples length: 628
made it out the get_samples match (GOOD)
Alpha: 1.3323146843717708; Min: 7297.524571344659; Max: 1728867.839564542
Q: 102.0586384835913; n: 85275842.6555986
dQ/dn <= 0
S(k) = 1917716365582.7898/((k + 102.0586384835913)^3.0091959429673256); P(S) = 0 for S < 7297.524571344659, P(S) = 7.627141765491546/(S^1.3323146843717708) for 7297.524571344659 <= S <= 1728867.839564542, and P(S) = 0 for S > 1728867.839564542; S(1) + ... + S(628) = 85275842.6555986; v = 17.108415368021717

i: 5; samples length: 785
made it out the get_samples match (GOOD)
Alpha: 1.3671561682904585; Min: 7779.761722743566; Max: 1510017.7301565341
Q: 113.45158316059126; n: 95834019.93593551
dQ/dn > 0
S(k) = 596402450901.2389/((k + 113.45158316059126)^2.7236366602695794); P(S) = 0 for S < 7779.761722743566, P(S) = 11.514244096422479/(S^1.3671561682904585) for 7779.761722743566 <= S <= 1510017.7301565341, and P(S) = 0 for S > 1510017.7301565341; S(1) + ... + S(785) = 95834019.93593551; v = 22.34343144475202

i: 6; samples length: 942
made it out the get_samples match (GOOD)
Alpha: 1.4145951060129034; Min: 6442.320697619271; Max: 1510006.1665371282
Q: 98.05886853560477; n: 100379211.57730357
dQ/dn <= 0
S(k) = 96034827134.81454/((k + 98.05886853560477)^2.4119918095918798); P(S) = 0 for S < 6442.320697619271, P(S) = 17.561970087404124/(S^1.4145951060129034) for 6442.320697619271 <= S <= 1510006.1665371282, and P(S) = 0 for S > 1510006.1665371282; S(1) + ... + S(942) = 100379211.57730357; v = 27.96002989348096

i: 7; samples length: 1100
made it out the get_samples match (GOOD)
Alpha: 1.4503126652934673; Min: 5976.665517512022; Max: 1469625.8002926677
Q: 92.21650616068426; n: 105412154.0185488
dQ/dn <= 0
S(k) = 33916937690.439926/((k + 92.21650616068426)^2.220679268144287); P(S) = 0 for S < 5976.665517512022, P(S) = 24.667600252280344/(S^1.4503126652934673) for 5976.665517512022 <= S <= 1469625.8002926677, and P(S) = 0 for S > 1469625.8002926677; S(1) + ... + S(1100) = 105412154.0185488; v = 33.67570501493068

i: 8; samples length: 1257
made it out the get_samples match (GOOD)
Alpha: 1.4232542026031754; Min: 6677.433100518249; Max: 1321072.9578712515
Q: 134.09400068414374; n: 123981833.79965305
dQ/dn > 0
S(k) = 140359802860.10886/((k + 134.09400068414374)^2.362646357318172); P(S) = 0 for S < 6677.433100518249, P(S) = 19.695872221877277/(S^1.4232542026031754) for 6677.433100518249 <= S <= 1321072.9578712515, and P(S) = 0 for S > 1321072.9578712515; S(1) + ... + S(1257) = 123981833.79965305; v = 38.94877796014249

i: 9; samples length: 1414
made it out the get_samples match (GOOD)
Alpha: 1.4385085278201382; Min: 6760.9011073553775; Max: 1323801.5633870023
Q: 139.7872425321228; n: 137244697.3516329
dQ/dn > 0
S(k) = 103387998734.15765/((k + 139.7872425321228)^2.2804573607064884); P(S) = 0 for S < 6760.9011073553775, P(S) = 23.263506459695023/(S^1.4385085278201382) for 6760.9011073553775 <= S <= 1323801.5633870023, and P(S) = 0 for S > 1323801.5633870023; S(1) + ... + S(1414) = 137244697.3516329; v = 44.20120910854812

i: 10; samples length: 1571
made it out the get_samples match (GOOD)
Alpha: 1.472464352522359; Min: 6768.908878821133; Max: 1308329.6933787954
Q: 130.625773964106; n: 143699764.0440469
dQ/dn <= 0
S(k) = 39393365389.7855/((k + 130.625773964106)^2.1165617991310266); P(S) = 0 for S < 6768.908878821133, P(S) = 33.25472893932779/(S^1.472464352522359) for 6768.908878821133 <= S <= 1308329.6933787954, and P(S) = 0 for S > 1308329.6933787954; S(1) + ... + S(1571) = 143699764.0440469; v = 49.42791282854621

i: 11; samples length: 1728
made it out the get_samples match (GOOD)
Alpha: 1.413703202535572; Min: 5936.898615856615; Max: 1038747.7548656921
Q: 203.99895720281904; n: 142827334.93420938
dQ/dn <= 0
S(k) = 397487884817.9642/((k + 203.99895720281904)^2.4171918270659645); P(S) = 0 for S < 5936.898615856615, P(S) = 17.075800089338323/(S^1.413703202535572) for 5936.898615856615 <= S <= 1038747.7548656921, and P(S) = 0 for S > 1038747.7548656921; S(1) + ... + S(1728) = 142827334.93420938; v = 54.45873961946982

i: 12; samples length: 1885
made it out the get_samples match (GOOD)
Alpha: 1.3618369479134296; Min: 6466.615009853045; Max: 1045730.4324971059
Q: 299.29993494485643; n: 171613434.1790534
dQ/dn > 0
S(k) = 7287449943846.414/((k + 299.29993494485643)^2.763675754415363); P(S) = 0 for S < 6466.615009853045, P(S) = 10.290748379797318/(S^1.3618369479134296) for 6466.615009853045 <= S <= 1045730.4324971059, and P(S) = 0 for S > 1045730.4324971059; S(1) + ... + S(1885) = 171613434.1790534; v = 59.33728411501734

i: 13; samples length: 2042
made it out the get_samples match (GOOD)
Alpha: 1.3215241079065438; Min: 7335.880514424126; Max: 880835.0698653716
Q: 437.9907026596944; n: 177677531.512916
dQ/dn > 0
S(k) = 144658360341242.38/((k + 437.9907026596944)^3.110186687122902); P(S) = 0 for S < 7335.880514424126, P(S) = 7.159760325081992/(S^1.3215241079065438) for 7335.880514424126 <= S <= 880835.0698653716, and P(S) = 0 for S > 880835.0698653716; S(1) + ... + S(2042) = 177677531.512916; v = 63.66136459554664

i: 14; samples length: 2200
made it out the get_samples match (GOOD)
Alpha: 1.308229913059189; Min: 8354.869674258445; Max: 813503.0258571046
Q: 536.4537844787798; n: 189025479.28807998
dQ/dn > 0
S(k) = 583266044302772.5/((k + 536.4537844787798)^3.2443314475061062); P(S) = 0 for S < 8354.869674258445, P(S) = 6.593586689857201/(S^1.308229913059189) for 8354.869674258445 <= S <= 813503.0258571046, and P(S) = 0 for S > 813503.0258571046; S(1) + ... + S(2200) = 189025479.28807998; v = 67.61517667501994

i: 15; samples length: 2357
made it out the get_samples match (GOOD)
Alpha: 1.2715540364164015; Min: 11366.133767018631; Max: 760202.905908552
Q: 752.8135855678531; n: 208215870.80564865
dQ/dn > 0
S(k) = 29809521377878956/((k + 752.8135855678531)^3.6825083257705584); P(S) = 0 for S < 11366.133767018631, P(S) = 5.038242034789551/(S^1.2715540364164015) for 11366.133767018631 <= S <= 760202.905908552, and P(S) = 0 for S > 760202.905908552; S(1) + ... + S(2357) = 208215870.80564865; v = 70.94695795836576

i: 16; samples length: 2514
made it out the get_samples match (GOOD)
Alpha: 1.2656450066101586; Min: 11588.565449288491; Max: 717568.6073573623
Q: 840.1824086709705; n: 212982609.61009318
dQ/dn > 0
S(k) = 73188140045334370/((k + 840.1824086709705)^3.764422349814871); P(S) = 0 for S < 11588.565449288491, P(S) = 4.792344091300671/(S^1.2656450066101586) for 11588.565449288491 <= S <= 717568.6073573623, and P(S) = 0 for S > 717568.6073573623; S(1) + ... + S(2514) = 212982609.61009318; v = 74.15762615592736

i: 17; samples length: 2671
made it out the get_samples match (GOOD)
Alpha: 1.2616997620986068; Min: 11403.29054410065; Max: 720130.6661795683
Q: 902.6400067765092; n: 225300653.8629612
dQ/dn > 0
S(k) = 141560823032785900/((k + 902.6400067765092)^3.8211727514800202); P(S) = 0 for S < 11403.29054410065, P(S) = 4.556489463642735/(S^1.2616997620986068) for 11403.29054410065 <= S <= 720130.6661795683, and P(S) = 0 for S > 720130.6661795683; S(1) + ... + S(2671) = 225300653.8629612; v = 77.39899831716566

i: 18; samples length: 2828
made it out the get_samples match (GOOD)
Alpha: 1.2508951834870228; Min: 12507.084439092023; Max: 688737.3602119803
Q: 1034.4192463942456; n: 233603231.39060718
dQ/dn > 0
S(k) = 714191648889149300/((k + 1034.4192463942456)^3.985728167841547); P(S) = 0 for S < 12507.084439092023, P(S) = 4.218980530115096/(S^1.2508951834870228) for 12507.084439092023 <= S <= 688737.3602119803, and P(S) = 0 for S > 688737.3602119803; S(1) + ... + S(2828) = 233603231.39060718; v = 80.42972263088872

i: 19; samples length: 2985
made it out the get_samples match (GOOD)
Alpha: 1.2628792886918405; Min: 11433.541749173512; Max: 694877.2805944488
Q: 1014.003288852495; n: 245578321.8058929
dQ/dn <= 0
S(k) = 189218685784455580/((k + 1014.003288852495)^3.8040273350413965); P(S) = 0 for S < 11433.541749173512, P(S) = 4.6432972218320625/(S^1.2628792886918405) for 11433.541749173512 <= S <= 694877.2805944488, and P(S) = 0 for S > 694877.2805944488; S(1) + ... + S(2985) = 245578321.8058929; v = 83.61140582043923

i: 20; samples length: 3143
made it out the get_samples match (GOOD)
Alpha: 1.268655199379126; Min: 11363.23140896701; Max: 674898.1148222779
Q: 1049.1096565148778; n: 253770184.01837945
dQ/dn > 0
S(k) = 118431801119505940/((k + 1049.1096565148778)^3.7222432408196235); P(S) = 0 for S < 11363.23140896701, P(S) = 4.9558408982828395/(S^1.268655199379126) for 11363.23140896701 <= S <= 674898.1148222779, and P(S) = 0 for S > 674898.1148222779; S(1) + ... + S(3143) = 253770184.01837945; v = 86.75754650936872

dQ/dn <= 0: 7/19
```

# Disclaimer

This code works for a range of use cases, but can have lower accuracy as the non-linearity of the system increases, or as the data set size decreases. For these use cases, a potential solution to this problem might be finding sufficiently good hyperparameters for the regularization terms used in this code's version of the Levenberg-Marquardt algorithm.

# Acknowledgement

This work was performed during a summer research internship advised by Professor Chen Ding. Dylan McKellips pointed me to the "Dynamical approach to Zipf’s law" paper that this work is based on and provided advice for writing the code. This work was funded by an NSF REU grant.
